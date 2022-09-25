extern crate wee_alloc;

use serde::{Serialize, Deserialize};

pub mod types;

use chrono::NaiveDateTime;
use std::collections::HashMap;
use tonic_web_wasm_client::Client;
use wasm_bindgen::prelude::*;

use types::outages::ListOutagesRequest;
use types::outages::customer_outages_client::CustomerOutagesClient;

const BASE_URL: &str = "http://127.0.0.1:5000";

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize, Clone, Copy)]
struct OutagePoint {
    datetime: NaiveDateTime,
    outages: i64
}

#[wasm_bindgen]
pub async fn get_outages() -> Result<JsValue, JsValue> {
    let mut query_client: CustomerOutagesClient<Client> = CustomerOutagesClient::new(Client::new(BASE_URL.to_owned()));
    let request: ListOutagesRequest = ListOutagesRequest{};
    let mut stream = query_client.list_outages(request).await.unwrap().into_inner();

    let mut result: HashMap<String, Vec<OutagePoint>> = HashMap::new();
    while let Some(outage) = stream.message().await.map_err(|e| e.message().to_owned())? {
        let this_datetime = outage.datetime.unwrap();
        let this_point = OutagePoint{datetime: NaiveDateTime::from_timestamp(this_datetime.seconds, 0), outages: outage.outages };
        result.entry(outage.area_name).and_modify(|e| e.push(this_point)).or_insert(vec![this_point]);
    }

    Ok(serde_wasm_bindgen::to_value(&result).unwrap())
}