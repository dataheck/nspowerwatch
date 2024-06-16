use chrono::{Duration, Utc};
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
            error!("Unable to obtain database connection: {:?}", e);
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

        let earliest_data = Utc::now().naive_utc() - Duration::days(90);

        // todo: this is not actually async yet
        // I believe this sorting will make plotting more efficient on the client-side.
        let all_outages: Vec<Outages>  = match outages::table.filter(outages::datetime.gt(earliest_data)).order((outages::area_name, outages::datetime.asc())).load::<Outages>(&mut connection) {
            Ok(r) => r,
            Err(e) => {
                error!("Error loading outages: {}", e);
                return Err(Status::internal("Encountered an error when loading outages."));
            }
        };

        tokio::spawn(async move {
            for outage in &all_outages {
                let result = tx.send(Ok(OutageStreamItem{
                    area_name: outage.area_name.clone(),
                    outages: outage.customers_affected,
                    datetime: Some(Timestamp{
                        seconds: outage.datetime.and_utc().timestamp(),
                        nanos: 0
                    })
                })).await;

                if result.is_err() {
                    error!("Error sending outage: {:?}", result);
                    break;
                }
            }            
        });        

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}