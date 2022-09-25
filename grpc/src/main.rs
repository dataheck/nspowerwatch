use clap::Parser;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use log::info;
use tonic::transport::Server;

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

    info!("Server started. gRPC listening at {}", addr);

    let pool = get_connection_pool();

    let outage_service_state = MyOutageServiceServer{pool: pool.clone()};    

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(CustomerOutagesServer::new(outage_service_state)))
        .serve(addr)
        .await?;

    Ok(())
}
