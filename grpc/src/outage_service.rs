use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use log::error;
use prost_types::Timestamp;
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc;
use tonic::{Request, Response, Status};

fn get_connection_from_pool(pool: &Pool<ConnectionManager<PgConnection>>) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Status> {
    match pool.get() {
        Ok(c) => Ok(c),
        Err(e) => {
            error!("Unable to obtain datbase connection: {:?}", e);
            Err(Status::resource_exhausted("No database connections are available."))
        }
    }
}
use crate::types;
use types::outages::{
    ListOutagesRequest, OutageStreamItem
};
use types::outages::customer_outages_server::CustomerOutages;
use backend::models::Outages;

pub struct MyOutageServiceServer {
    pub pool: Pool<ConnectionManager<PgConnection>>
}

#[tonic::async_trait]
impl CustomerOutages for MyOutageServiceServer {
    type ListOutagesStream = ReceiverStream<Result<OutageStreamItem, Status>>;
    async fn list_outages(&self, _request: Request<ListOutagesRequest>) -> Result<Response<Self::ListOutagesStream>, Status> {
        use backend::schema::outages;

        let mut connection = get_connection_from_pool(&self.pool)?;

        let (tx, rx) = mpsc::channel(4);

        // todo: this is not actually async yet
        let all_outages = match outages::table.load::<Outages>(&mut connection) {
            Ok(r) => r,
            Err(e) => {
                error!("Error loading outages: {}", e);
                return Err(Status::internal("Encountered an error when loading outages."));
            }
        };

        tokio::spawn(async move {
            for outage in &all_outages {
                tx.send(Ok(OutageStreamItem{
                    area_name: outage.area_name.clone(),
                    outages: outage.customers_affected,
                    datetime: Some(Timestamp{
                        seconds: outage.datetime.timestamp(),
                        nanos: 0
                    })
                })).await.unwrap();
            }            
        });        

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}