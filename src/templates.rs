use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use reqwest::StatusCode;

use crate::{news::NewsItem, residents::Residents, transportation::Arrivals, weather::WeatherData};

pub struct HtmlTemplate<T>(pub T);

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error: {err}"),
            )
                .into_response(),
        }
    }
}

#[derive(Debug)]
pub struct HousingCompany {
    pub name: String,
    pub address: String,
}


mod filters {

    const FINNISH_TIMEZONE: chrono::FixedOffset = chrono::FixedOffset::east_opt(2 * 3600).unwrap(); // UTC+2

    /// Round a value to the nearest integer
    pub fn round<T: std::fmt::Display>(value: T) -> ::askama::Result<String> {
        if value.to_string().is_empty() {
            return Ok(value.to_string());
        }
        
        let value = value.to_string().parse::<f64>();
        let rounded_value = value.unwrap().round();
        Ok(rounded_value.to_string())
    }

    /// Explain the weather symbol code
    pub fn symbol_code_desc<T: std::fmt::Display>(value: T) -> ::askama::Result<String> {
        let symbol_code = value.to_string();
        let description = match symbol_code.as_str() {
            "clear_sky" => "Clear sky",
            "cloudy" => "Cloudy",
            "fair" => "Fair",
            "fog" => "Fog",
            "heavyrain" => "Heavy rain",
            "heavyrainandthunder" => "Heavy rain and thunder",
            "heavyrainshowers" => "Heavy rain showers",
            "heavyrainshowersandthunder" => "Heavy rain showers and thunder",
            "heavysleet" => "Heavy sleet",
            "heavysleetandthunder" => "Heavy sleet and thunder",
            "heavysleetshowers" => "Heavy sleet showers",
            "heavysleetshowersandthunder" => "Heavy sleet showers and thunder",
            "heavysnow" => "Heavy snow",
            "heavysnowandthunder" => "Heavy snow and thunder",
            "heavysnowshowers" => "Heavy snow showers",
            "heavysnowshowersandthunder" => "Heavy snow showers and thunder",
            "lightrain" => "Light rain",
            "lightrainandthunder" => "Light rain and thunder",
            "lightrainshowers" => "Light rain showers",
            "lightrainshowersandthunder" => "Light rain showers and thunder",
            "lightsleet" => "Light sleet",
            "lightsleetandthunder" => "Light sleet and thunder",
            "lightsleetshowers" => "Light sleet showers",
            "lightsnow" => "Light snow",
            "lightsnowandthunder" => "Light snow and thunder",
            "lightsnowshowers" => "Light snow showers",
            "lightssleetshowersandthunder" => "Light sleet showers and thunder",
            "partlycloudy" => "Partly cloudy",
            "rain" => "Rain",
            "rainandthunder" => "Rain and thunder",
            "rainshowers" => "Rain showers",
            "rainshowersandthunder" => "Rain showers and thunder",
            "sleet" => "Sleet",
            "sleetandthunder" => "Sleet and thunder",
            "sleetshowers" => "Sleet showers",
            "sleetshowersandthunder" => "Sleet showers and thunder",
            "snow" => "Snow",
            "snowandthunder" => "Snow and thunder",
            "snowshowers" => "Snow showers",
            "snowshowersandthunder" => "Snow showers and thunder",
            _ => "Unknown weather"
        };
        Ok(description.to_string())
    }

    /// Parse ISO date to a finnish formatted date
    pub fn iso_to_fi<T: std::fmt::Display>(value: T) -> ::askama::Result<String> {
        if value.to_string().is_empty() {
            return Ok(value.to_string());
        }

        let updated_at = match chrono::DateTime::parse_from_rfc3339(&value.to_string()) {
            Ok(datetime) => datetime,
            Err(err) => {
                println!("Error parsing updated_at: {:?}", err);
                return Ok("".to_string())
            }
        };

    
        // Convert the updated_at field to Finnish timezone
        let updated_at_finnish = updated_at.with_timezone(&FINNISH_TIMEZONE);
        Ok(updated_at_finnish.format("%d.%m.%Y %H:%M:%S").to_string())
    }
}

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub housing_company: HousingCompany,
    pub next_arrivals: Vec<Arrivals>,
    pub residents: Vec<Residents>,
    pub weather: WeatherData,
    pub news: Vec<NewsItem>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Template)]
    #[template(source = "{{ test_value|round }}", ext = "html")]
    struct RoundingTemplate<'value> {
        test_value: &'value str,
    }
    
    #[test]
    fn test_round_template_specimen_1() {
        let t = RoundingTemplate { test_value: "50.12" };
        assert_eq!(t.render().unwrap(), "50");
    }

    #[test]
    fn test_round_template_specimen_2() {
        let t = RoundingTemplate { test_value: "50.12345" };
        assert_eq!(t.render().unwrap(), "50");
    }

    #[test]
    fn test_round_template_with_empty_string() {
        let t = RoundingTemplate { test_value: "" };
        assert_eq!(t.render().unwrap(), "");
    }
    #[derive(Template)]
    #[template(source = "{{ test_value|iso_to_fi }}", ext = "html")]
    struct IsoToFiTemplate<'value> {
        test_value: &'value str,
    }

    #[test]
    fn test_iso_to_fi_template_valid_date() {
        let t = IsoToFiTemplate { test_value: "2023-10-05T14:48:00.000Z" };
        assert_eq!(t.render().unwrap(), "05.10.2023 16:48:00");
    }

    #[test]
    fn test_iso_to_fi_template_invalid_date() {
        let t = IsoToFiTemplate { test_value: "invalid-date" };
        assert_eq!(t.render().unwrap(), "");
    }

    #[test]
    fn test_iso_to_fi_template_empty_string() {
        let t = IsoToFiTemplate { test_value: "" };
        assert_eq!(t.render().unwrap(), "");
    }

    #[derive(Template)]
    #[template(source = "{{ test_value|symbol_code_desc }}", ext = "html")]
    struct SymbolCodeTemplate<'value> {
        test_value: &'value str,
    }

    #[test]
    fn test_symbol_code_template_clear_sky() {
        let t = SymbolCodeTemplate { test_value: "clear_sky" };
        assert_eq!(t.render().unwrap(), "Clear sky");
    }

    #[test]
    fn test_symbol_code_template_heavyrain() {
        let t = SymbolCodeTemplate { test_value: "heavyrain" };
        assert_eq!(t.render().unwrap(), "Heavy rain");
    }

    #[test]
    fn test_symbol_code_template_partlycloudy() {
        let t = SymbolCodeTemplate { test_value: "partlycloudy" };
        assert_eq!(t.render().unwrap(), "Partly cloudy");
    }

    #[test]
    fn test_symbol_code_template_unknown() {
        let t = SymbolCodeTemplate { test_value: "unknown_code" };
        assert_eq!(t.render().unwrap(), "Unknown weather");
    }
}