use crate::config::CONFIG;
use chrono::TimeZone;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    #[serde(rename = "shortName")]
    short_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trip {
    route: Route,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopTimesWithoutPatterns {
    #[serde(rename = "realtimeArrival")]
    realtime_arrival: i64,
    #[serde(rename = "scheduledArrival")]
    scheduled_arrival: i64,
    #[serde(rename = "scheduledDeparture")]
    scheduled_departure: i64,
    headsign: Option<String>,
    trip: Trip,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "stopsByRadius")]
    stops_by_radius: StopsByRadius,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stop {
    name: String,
    desc: String,
    #[serde(rename = "gtfsId")]
    gtfs_id: String,
    #[serde(rename = "platformCode")]
    platform_code: Option<String>,
    #[serde(rename = "stoptimesWithoutPatterns")]
    stop_times_without_patterns: Vec<StopTimesWithoutPatterns>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    stop: Stop,
    distance: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Edge {
    pub node: Node,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StopsByRadius {
    edges: Vec<Edge>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    data: Data,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct Arrivals {
    pub gtfs_id: String,
    pub stop_name: String,
    pub stop_desc: String,
    pub route_short_name: String,
    pub headsign: String,
    pub realtime_arrival: i64,
    pub realtime_text: String,
}

const TWO_MINUTES: i64 = 120;
const TEN_MINUTES: i64 = 600;

fn query_stops_by_radius_query(start_time: i64, time_range: Option<i64>) -> String {
    let time_range = time_range.unwrap_or(TEN_MINUTES);
    let query = format!(
        r#"{{
        stopsByRadius(lat:60.188374, lon:24.984065, radius:750) {{
            edges {{
                node {{
                    stop {{
                        gtfsId
                        platformCode
                        name
                        desc

                        stoptimesWithoutPatterns(numberOfDepartures: 10, startTime: {start_time}, timeRange: {time_range} ) {{
                            scheduledArrival
                            scheduledDeparture
                            realtimeArrival
                            trip {{
                                route {{
                                    shortName
                                }}
                            }}
                            headsign
                        }}
                    }}
                    distance
                }}
            }}
        }}
    }}"#,
        start_time = start_time,
        time_range = time_range
    );
    query
}

fn format_minutes_diff_from_now(seconds_since_midnight: i64) -> String {
    // Get the current time in seconds since midnight in Helsinki time
    let now = chrono::Local::now();
    let midnight = match now.date_naive().and_hms_opt(0, 0, 0) {
        Some(midnight) => midnight,
        None => return "Error".to_string(),
    };
    let midnight_local = chrono::Local.from_local_datetime(&midnight).unwrap();
    let now_seconds_since_midnight = (now - midnight_local).num_seconds();

    // Calculate the difference in seconds
    let diff_seconds = seconds_since_midnight - now_seconds_since_midnight;

    // Convert the difference to minutes
    let diff_minutes = diff_seconds / 60;

    // Format the string
    format!("{} minutes from now", diff_minutes)
}

fn parse_times_to_arrivals(data: Data) -> Vec<Arrivals> {
    let mut arrivals = Vec::new();

    for edge in data.stops_by_radius.edges {
        for stop_time in edge.node.stop.stop_times_without_patterns {
            let realtime_text = format_minutes_diff_from_now(stop_time.realtime_arrival);
            let item = Arrivals {
                gtfs_id: edge.node.stop.gtfs_id.to_string(),
                stop_desc: edge.node.stop.desc.to_string(),
                stop_name: edge.node.stop.name.to_string(),
                route_short_name: stop_time.trip.route.short_name.to_string(),
                headsign: match stop_time.headsign {
                    Some(headsign) => headsign,
                    None => "".to_string(),
                },
                realtime_arrival: stop_time.realtime_arrival,
                realtime_text: realtime_text,
            };

            arrivals.push(item);
        }
    }

    arrivals.sort_by(|a, b| a.realtime_arrival.cmp(&b.realtime_arrival));
    arrivals
}

pub async fn get_next_arrivals() -> Result<Vec<Arrivals>, Box<dyn Error>> {
    let unix_now = chrono::Utc::now().timestamp();
    let unix_in_two_minutes = unix_now + TWO_MINUTES;
    let client = reqwest::Client::new();
    let request = client
        .post("https://api.digitransit.fi/routing/v1/routers/hsl/index/graphql")
        .header(
            "digitransit-subscription-key",
            &CONFIG.digitransit_subscription_key,
        )
        .json(
            &serde_json::json!({ "query": query_stops_by_radius_query(unix_in_two_minutes, None) }),
        );

    println!("{request:#?}");

    let response = match request.send().await {
        Ok(response) => {
            println!("{response:#?}");
            response
        }
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(Box::new(err));
        }
    };

    // first get text, easier to debug if something goes wrong
    let text = response.text().await.unwrap();

    // parse text into JSON
    let json_text: serde_json::Value = match serde_json::from_str(&text) {
        Ok(json) => {
            println!("{json:#?}");
            json
        }
        Err(err) => {
            println!("Error parsing JSON: {:?}", err);
            return Err(Box::new(err));
        }
    };

    // parse JSON into ApiResponse struct
    let json: ApiResponse = match serde_json::from_value(json_text) {
        Ok(json_response) => json_response,
        Err(err) => {
            println!("Error converting JSON to ApiResponse: {:?}", err);
            return Err(Box::new(err));
        }
    };

    let arrivals = parse_times_to_arrivals(json.data);

    Ok(arrivals)
}
