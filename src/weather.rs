use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Geometry {
    pub r#type: String,
    pub coordinates: Vec<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub r#type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Units {
    pub air_pressure_at_sea_level: String,
    pub air_temperature: String,
    pub air_temperature_max: String,
    pub air_temperature_min: String,
    pub air_temperature_percentile_10: String,
    pub air_temperature_percentile_90: String,
    pub cloud_area_fraction: String,
    pub cloud_area_fraction_high: String,
    pub cloud_area_fraction_low: String,
    pub cloud_area_fraction_medium: String,
    pub dew_point_temperature: String,
    pub fog_area_fraction: String,
    pub precipitation_amount: String,
    pub precipitation_amount_max: String,
    pub precipitation_amount_min: String,
    pub probability_of_precipitation: String,
    pub probability_of_thunder: String,
    pub relative_humidity: String,
    pub ultraviolet_index_clear_sky: String,
    pub wind_from_direction: String,
    pub wind_speed: String,
    pub wind_speed_of_gust: String,
    pub wind_speed_percentile_10: String,
    pub wind_speed_percentile_90: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub updated_at: String,
    pub units: Units,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Properties {
    pub meta: Meta,
    pub timeseries: Vec<Timeseries>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Timeseries {
    pub time: String,
    pub data: Data,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Summary {
    pub symbol_code: String,
    pub symbol_confidence: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Forecast {
    pub summary: Summary,
    pub details: Details,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Data {
    pub instant: Instant,
    pub next_1_hours: Option<Forecast>,
    pub next_6_hours: Option<Forecast>,
    pub next_12_hours: Option<Forecast>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Instant {
    pub details: Details,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub precipitation_amount: Option<f64>,
    pub precipitation_amount_max: Option<f64>,
    pub precipitation_amount_min: Option<f64>,
    pub probability_of_precipitation: Option<f64>,
    pub probability_of_thunder: Option<f64>,
}

pub const EMPTY_WEATHER_DATA: WeatherData = WeatherData {
    r#type: String::new(),
    properties: Properties {
        meta: Meta {
            updated_at: String::new(),
            units: Units {
                air_temperature: String::new(),
                air_temperature_max: String::new(),
                air_temperature_min: String::new(),
                air_pressure_at_sea_level: String::new(),
                air_temperature_percentile_10: String::new(),
                air_temperature_percentile_90: String::new(),
                cloud_area_fraction: String::new(),
                cloud_area_fraction_high: String::new(),
                cloud_area_fraction_low: String::new(),
                cloud_area_fraction_medium: String::new(),
                dew_point_temperature: String::new(),
                precipitation_amount: String::new(),
                precipitation_amount_max: String::new(),
                precipitation_amount_min: String::new(),
                probability_of_precipitation: String::new(),
                probability_of_thunder: String::new(),
                fog_area_fraction: String::new(),
                relative_humidity: String::new(),
                ultraviolet_index_clear_sky: String::new(),
                wind_from_direction: String::new(),
                wind_speed: String::new(),
                wind_speed_of_gust: String::new(),
                wind_speed_percentile_10: String::new(),
                wind_speed_percentile_90: String::new(),
            },
        },
        timeseries: vec![],
    },
    geometry: Geometry {
        coordinates: vec![],
        r#type: String::new(),
    },
};

pub async fn get_weather_data() -> Result<WeatherData, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let request = client
        .get("https://api.met.no/weatherapi/locationforecast/2.0/complete?lat=60.188374&lon=24.984065")
        .header("User-Agent", "reqwest");

    let response = match request.send().await {
        Ok(response) => response,
        Err(err) => {
            println!("Error: {:?}", err);
            return Err(Box::new(err));
        }
    };

    // first get text, easier to debug if something goes wrong
    let response_text = response.text().await.unwrap();

    // parse text into JSON
    let response_json: serde_json::Value = match serde_json::from_str(&response_text) {
        Ok(json) => json,
        Err(err) => {
            println!("Error parsing JSON: {:?}", err);
            return Err(Box::new(err));
        }
    };

    // parse JSON into WeatherData struct
    let data: WeatherData = match serde_json::from_value(response_json) {
        Ok(data) => data,
        Err(err) => {
            println!("Error converting JSON to WeatherData: {:?}", err);
            return Err(Box::new(err));
        }
    };

    Ok(data)
}
