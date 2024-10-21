use crate::config::CONFIG;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Route {
    #[serde(rename = "shortName")]
    short_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trip {
    route: Route,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Arrivals {
    pub route_short_name: String,
    pub headsign: String,
    pub arrival: String,
}

static GET_STOPS_BY_RADIUS_QUERY: &str = r#"{
        stopsByRadius(lat:60.188374, lon:24.984065, radius:750) {
            edges {
                node {
                    stop {
                        gtfsId
                        platformCode
                        name

                        stoptimesWithoutPatterns(numberOfDepartures: 10) {
                            scheduledArrival
                            scheduledDeparture
                            realtimeArrival
                            trip {
                                route {
                                    shortName
                                }
                            }
                            headsign
                        }
                    }
                    distance
                }
            }
        }
    }"#;

fn format_arrival_time(time: i64) -> String {
    let duration = chrono::Duration::milliseconds(time);
    let minutes = duration.num_minutes();
    let seconds = duration.num_seconds() % 60;

    if minutes > 0 {
        format!("in {} minutes", minutes)
    } else if seconds > 0 {
        format!("in {} seconds", seconds)
    } else {
        "now".to_string()
    }
}

fn parse_times_to_arrivals(data: Data) -> Vec<Arrivals> {
    let mut arrivals = Vec::new();
    for edge in data.stops_by_radius.edges {
        for stop_time in edge.node.stop.stop_times_without_patterns {
            let arrival = format_arrival_time(stop_time.realtime_arrival);

            let item = Arrivals {
                route_short_name: stop_time.trip.route.short_name.to_string(),
                headsign: match stop_time.headsign {
                    Some(headsign) => headsign,
                    None => "".to_string(),
                },
                arrival,
            };

            arrivals.push(item);
        }
    }

    arrivals
}

pub async fn get_next_arrivals() -> Result<Vec<Arrivals>, Box<dyn Error>> {
    let client = Client::new();
    let request = client
        .post("https://api.digitransit.fi/routing/v1/routers/hsl/index/graphql")
        .header(
            "digitransit-subscription-key",
            &CONFIG.digitransit_subscription_key,
        )
        .json(&serde_json::json!({ "query": GET_STOPS_BY_RADIUS_QUERY }));

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
