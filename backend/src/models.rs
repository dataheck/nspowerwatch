use crate::schema::*;

use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Clone)]
#[diesel(table_name = outages)]
pub struct Outages {
    pub id: i64,
    pub datetime: NaiveDateTime, // utc
    pub area_name: String,
    pub customers_affected: i64, 
}

#[derive(Insertable)]
#[diesel(table_name = outages)]
pub struct NewOutage {
    pub datetime: NaiveDateTime, // utc
    pub area_name: String,
    pub customers_affected: i64, 
}