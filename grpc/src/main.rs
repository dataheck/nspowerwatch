use std::env;
use std::time::Duration;

use clap::Parser;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use http::header::{HeaderValue, HeaderName};
use log::info;
use tonic_web::GrpcWebLayer;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tower_http::cors::CorsLayer;

use backend::get_database_url;

pub mod types;
pub mod outage_service;

use types::outages::customer_outages_server::CustomerOutagesServer;
use outage_service::MyOutageServiceServer;


#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CLIArgs {
    #[clap(short = 'a', long, value_parser, default_value = "127.0.0.1")]
    grpc_address: String,
    #[clap(short = 'p', long, value_parser, default_value_t = 5000)]
    grpc_port: u64,
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    let url = get_database_url();
    let manager = ConnectionManager::<PgConnection>::new(url);
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

const DEFAULT_EXPOSED_HEADERS: [&str; 3] = [
    "grpc-status", "grpc-message", "grpc-status-details-bin"
];

const DEFAULT_ALLOW_HEADERS: [&str; 7] = [
    "x-grpc-web", "content-type", "x-user-agent", "grpc-timeout", "origin", "host", "x-requested-with"
];

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let matches = CLIArgs::parse();
    let addr = format!("{}:{}", matches.grpc_address, matches.grpc_port).parse()?;

    dotenv().expect("Unable to load .env file.");

    let cert = (tokio::fs::read(env::var("CERT_PATH").expect("CERT_PATH must be set")).await).expect("CERT_PATH specified, but does not exist.");
    let key = (tokio::fs::read(env::var("KEY_PATH").expect("KEY_PATH must be set")).await).expect("KEY_PATH specified, but does not exist.");
    let identity = Identity::from_pem(cert, key);

    let timeout_secs: u64 = match env::var("TIMEOUT_SECS") {
        Ok(val) => val.parse().expect("TIMEOUT_SECS must be a number."),
        Err(_) => 30,
    };

    info!("Server started. gRPC listening at {}", addr);

    let pool = get_connection_pool();

    let outage_service_state = MyOutageServiceServer{pool: pool.clone()};    

    let allow_origin = match cfg!(debug_assertions) {
        true => "http://eqsplit.local:8000".parse::<HeaderValue>().unwrap(), // i.e.: trunk via local-ssl-proxy
        false => "https://outages.dataheck.com".parse::<HeaderValue>().unwrap(),
    }; 

    let cors = CorsLayer::new()
        .allow_origin(allow_origin).expose_headers(
            DEFAULT_EXPOSED_HEADERS
                .iter()
                .cloned()
                .map(HeaderName::from_static)
                .collect::<Vec<HeaderName>>(),
        )
        .allow_headers(
            DEFAULT_ALLOW_HEADERS
                .iter()
                .cloned()
                .map(HeaderName::from_static)
                .collect::<Vec<HeaderName>>(),
        );

    Server::builder()
        .timeout(Duration::from_secs(timeout_secs))
        .accept_http1(true)
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .layer(cors)
        .layer(GrpcWebLayer::new())
        .add_service(CustomerOutagesServer::new(outage_service_state))
        .serve(addr)
        .await?;

    Ok(())
}
