use prometheus::{self, Encoder, IntCounter, TextEncoder};

use lazy_static::lazy_static;
use prometheus::register_int_counter;
use warp::Filter;

lazy_static! {
    static ref REQUEST_COUNTER: IntCounter =
        register_int_counter!("silly_requests", "Number of silly requests received").unwrap();
}

fn silly_endpoint() -> &'static str {
    REQUEST_COUNTER.inc();
    return "my silly endpoint";
}

fn metrics() -> String {
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();

    // Gather the metrics.
    let metric_families = prometheus::gather();
    // Encode them to send.
    encoder.encode(&metric_families, &mut buffer).unwrap();
    return String::from_utf8(buffer.clone()).unwrap();
}

#[tokio::main]
async fn main() {
    // GET /silly => 200 OK with body "my silly endpoint"
    let my_silly_endpoint = warp::path::path("silly").map(silly_endpoint);

    // GET /metrics for prometheus key/value metrics about the server
    let metrics = warp::path::path("metrics").map(metrics);

    let routes = warp::get().and(my_silly_endpoint).or(metrics);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
