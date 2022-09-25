table! {
    outages (id) {
        id -> Int8,
        datetime -> Timestamp,
        area_name -> Text,
        customers_affected -> Int8,
    }
}
