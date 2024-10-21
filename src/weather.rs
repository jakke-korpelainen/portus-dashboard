use std::error::Error;

use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_CALL: &str =
    "https://api.met.no/weatherapi/locationforecast/2.0/complete?lat=60.188374&lon=24.984065";

#[derive(Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    r#type: String,
    geometry: Geometry,
    pub properties: Properties,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Units {
    pub air_temperature: String,
    pub air_pressure_at_sea_level: String,
    pub air_temperature_percentile_10: String,
    pub air_temperature_percentile_90: String,
    pub cloud_area_fraction: String,
    pub cloud_area_fraction_high: String,
    pub cloud_area_fraction_low: String,
    pub cloud_area_fraction_medium: String,
    pub dew_point_temperature: String,
    pub fog_area_fraction: String,
    pub relative_humidity: String,
    pub ultraviolet_index_clear_sky: String,
    pub wind_from_direction: String,
    pub wind_speed: String,
    pub wind_speed_of_gust: String,
    pub wind_speed_percentile_10: String,
    pub wind_speed_percentile_90: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    pub updated_at: String,
    pub units: Units,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Timeseries>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Timeseries {
    pub time: String,
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub symbol_code: String,
    pub symbol_confidence: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextHours {
    pub summary: Summary,
    pub details: Details,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub instant: Instant,
    pub next_1_hours: Option<NextHours>,
    pub next_6_hours: Option<NextHours>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Instant {
    pub details: Details,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Details {
    pub air_temperature: Option<f64>,
    pub air_pressure_at_sea_level: Option<f64>,
    pub air_temperature_percentile_10: Option<f64>,
    pub air_temperature_percentile_90: Option<f64>,
    pub cloud_area_fraction: Option<f64>,
    pub cloud_area_fraction_high: Option<f64>,
    pub cloud_area_fraction_low: Option<f64>,
    pub cloud_area_fraction_medium: Option<f64>,
    pub dew_point_temperature: Option<f64>,
    pub fog_area_fraction: Option<f64>,
    pub relative_humidity: Option<f64>,
    pub ultraviolet_index_clear_sky: Option<f64>,
    pub wind_from_direction: Option<f64>,
    pub wind_speed: Option<f64>,
    pub wind_speed_of_gust: Option<f64>,
    pub wind_speed_percentile_10: Option<f64>,
    pub wind_speed_percentile_90: Option<f64>,
}

pub async fn get_weather_data() -> Result<WeatherData, Box<dyn Error>> {
    let client = Client::new();
    let request = client.get(API_CALL).header("User-Agent", "reqwest");

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

    // parse JSON into WeatherData struct
    let data: WeatherData = match serde_json::from_value(json_text) {
        Ok(json_response) => json_response,
        Err(err) => {
            println!("Error converting JSON to WeatherData: {:?}", err);
            return Err(Box::new(err));
        }
    };

    Ok(data)
}
