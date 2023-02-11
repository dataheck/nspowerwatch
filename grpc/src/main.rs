use clap::Parser;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use log::info;
use std::env;
use tonic::transport::{Identity, Server, ServerTlsConfig};
use tonic_web::GrpcWebLayer;

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

#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    let matches = CLIArgs::parse();
    let addr = format!("{}:{}", matches.grpc_address, matches.grpc_port).parse()?;

    dotenv().ok();
    let cert = (tokio::fs::read(env::var("CERT_PATH").expect("CERT_PATH must be set")).await).expect("CERT_PATH specified, but does not exist.");
    let key = (tokio::fs::read(env::var("KEY_PATH").expect("KEY_PATH must be set")).await).expect("KEY_PATH specified, but does not exist.");
    let identity = Identity::from_pem(cert, key);

    info!("Server started. gRPC listening at {}", addr);

    let pool = get_connection_pool();

    let outage_service_state = MyOutageServiceServer{pool: pool.clone()};    

    Server::builder()
        //.accept_http1(true)
        .tls_config(ServerTlsConfig::new().identity(identity))?
        .layer(GrpcWebLayer::new())
        .add_service(CustomerOutagesServer::new(outage_service_state))
        .serve(addr)
        .await?;

    Ok(())
}
