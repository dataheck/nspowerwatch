extern crate chrono;
extern crate wee_alloc;

pub mod types;

use chrono::NaiveDateTime;
use std::collections::HashMap;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

use types::outages::ListOutagesRequest;
use types::outages::customer_outages_client::CustomerOutagesClient;

const BASE_URL: &str = "https://outages.dataheck.com:5000";

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub async fn get_outages() -> Result<JsValue, JsValue> {
    let mut query_client: CustomerOutagesClient<Client> = CustomerOutagesClient::new(Client::new(BASE_URL.to_owned()));
    let request: ListOutagesRequest = ListOutagesRequest{};
    let mut stream = query_client.list_outages(request).await.unwrap().into_inner();

    let mut all_datetimes = Vec::new();
    let mut all_outages: HashMap<String, Vec<i64>> = HashMap::new();

    while let Some(outage) = stream.message().await.map_err(|e| e.message().to_owned())? {
        let timestamp = { 
            let grpc_format = outage.datetime.unwrap();
            let this_ndatetime = NaiveDateTime::from_timestamp(grpc_format.seconds, 0);
            this_ndatetime.timestamp()*1000
        };

        if !all_datetimes.contains(&timestamp) {
            all_datetimes.push(timestamp);
        }

        all_outages.entry(outage.area_name).and_modify(|e| e.push(outage.outages)).or_insert(vec![outage.outages]);
    }

    let result = (all_datetimes, all_outages);
    Ok(serde_wasm_bindgen::to_value(&result).unwrap())
}