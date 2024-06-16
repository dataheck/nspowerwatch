extern crate chrono;
extern crate wee_alloc;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

pub mod types;
use types::outages::ListOutagesRequest;
use types::outages::customer_outages_client::CustomerOutagesClient;

const BASE_URL: &str = match cfg!(debug_assertions) {
    false => "https://outages.dataheck.com:5000",
    true => "http://127.0.0.1:5000"
};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
struct Point {
    pub x: i64,
    pub y: i64
}

#[wasm_bindgen]
pub async fn get_outages() -> Result<JsValue, JsValue> {
    let mut query_client: CustomerOutagesClient<Client> = CustomerOutagesClient::new(Client::new(BASE_URL.to_owned()));
    let request: ListOutagesRequest = ListOutagesRequest{};
    let mut stream = query_client.list_outages(request).await.unwrap().into_inner();

    let mut all_datetimes = Vec::new();
    let mut all_outages: HashMap<String, Vec<Point>> = HashMap::new();

    while let Some(outage) = stream.message().await.map_err(|e| e.message().to_owned())? {
        let timestamp: i64 = { 
            let grpc_format = outage.datetime.unwrap();
            let this_ndatetime = DateTime::<Utc>::from_timestamp(grpc_format.seconds, grpc_format.nanos as u32).unwrap();
            
            this_ndatetime.timestamp()*1000
        };

        if !all_datetimes.contains(&timestamp) {
            all_datetimes.push(timestamp);
        }

        all_outages.entry(outage.area_name)
            .and_modify(|e| e.push(Point{x: timestamp, y: outage.outages}))
            .or_insert(vec![Point{x: timestamp, y: outage.outages}]);
    }

    Ok(serde_wasm_bindgen::to_value(&all_outages).unwrap())
}