use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::insert_into;
use std::collections::HashMap;
use log::{error, info};
use serde::{Serialize, Deserialize};
use simple_logger::SimpleLogger;

use backend::get_database_url;
use backend::models::NewOutage;

const BASE_URL: &str = "http://outagemap.nspower.ca/resources/data/external/interval_generation_data";
const METADATA_FILENAME: &str = "metadata.json";
const SUMMARY_FILENAME: &str = "report_servicearea.json";


#[derive(Serialize, Deserialize)]
struct DirectoryResponse {
    directory: String
}

#[derive(Serialize, Deserialize, Debug)]
struct CustomerDetail {
    val: i64,
    masked: Option<bool>
}

#[derive(Serialize, Deserialize, Debug)]
struct SummaryArea {
    cust_a: CustomerDetail,
    etr: String,
    area_name: String,
    areas: Option<Vec<SummaryArea>>
}

#[derive(Serialize, Deserialize, Debug)]
struct SummaryResponse {
    // same as SUMMARY_FILENAME
    file_title: String, 
    file_data: HashMap<String, Vec<SummaryArea>>
}


fn fetch_directory() -> Result<String, ()> {
    // todo async
    let body = match reqwest::blocking::get(format!("{BASE_URL}/{METADATA_FILENAME}")) {
        Ok(r) => match r.text() {
            Ok(t) => t,
            Err(e) => {
                error!("Unable to get text from response: {}", e);
                return Err(())
            }
        },
        Err(e) => {
            error!("Unable to get response from server: {}", e);
            return Err(())
        }
    };

    let response: DirectoryResponse = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            error!("Unable to parse server response: {}", e);
            return Err(())
        }
    };

    Ok(response.directory)
}


fn fetch_summary(directory: &str) -> Result<SummaryResponse, ()>{
    let url = format!("{BASE_URL}/{directory}/{SUMMARY_FILENAME}");
    
    // todo async
    let body = match reqwest::blocking::get(url) {
        Ok(r) => match r.text() {
            Ok(t) => t,
            Err(e) => {
                error!("Unable to get text from response: {}", e);
                return Err(())
            }
        },
        Err(e) => {
            error!("Unable to get response from server: {}", e);
            return Err(())
        }
    };

    let response: SummaryResponse = match serde_json::from_str(&body) {
        Ok(r) => r,
        Err(e) => {
            error!("Unable to parse server response: {}", e);
            return Err(())
        }
    };

    Ok(response)
}

fn filename_to_datetime(filename: &str) -> Result<NaiveDateTime, ()> {
    match NaiveDateTime::parse_from_str(filename, "%Y_%m_%d_%H_%M_%S") {
        Ok(dt) => Ok(dt),
        Err(e) => {
            error!("Unable to parse filename as datetime: {}", e);
            Err(())
        }
    }
}


fn main() {
    use backend::schema::outages;

    SimpleLogger::new().init().unwrap();

    drop_root::drop_privileges();

    let directory = fetch_directory().unwrap();
    let this_datetime = filename_to_datetime(&directory).unwrap();

    // check if we already have data that matches this datetime
    let mut connection = PgConnection::establish(&get_database_url()).unwrap_or_else(|_| panic!("Error connecting to database"));
    match outages::table.filter(outages::datetime.eq(this_datetime)).count().get_result::<i64>(&mut connection) {
        // no data present, insert it
        Ok(0) => {
            info!("Found directory that should contain data we don't have yet");
            let summary = fetch_summary(&directory).unwrap();
            let nova_scotia = summary.file_data.get("areas").unwrap().first().unwrap();

            let mut outages_list = Vec::new();
            outages_list.push(NewOutage{
                area_name: String::from("NOVA SCOTIA"),
                datetime: this_datetime,
                customers_affected: nova_scotia.cust_a.val
            });
        
            if let Some(sub_areas) = nova_scotia.areas.as_ref() {
                for area in sub_areas {
                    outages_list.push(NewOutage { datetime: this_datetime, area_name: area.area_name.to_owned(), customers_affected: area.cust_a.val })
                }
            }
            info!("Attempting insertion.");
            insert_into(outages::table).values(&outages_list).execute(&mut connection).unwrap();
            info!("Data has been inserted.");
        },
        Ok(_count) => {
            info!("Data for datetime {} already in database.", this_datetime);
        },
        Err(e) => {
            panic!("Unable to fetch count of existing entries: {}", e)
        }
    }
}
