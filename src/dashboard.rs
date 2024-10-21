use axum::response::IntoResponse;
use residents::get_residents;
use templates::{DashboardTemplate, HousingCompany, HtmlTemplate};
use weather::get_weather_data;

use super::*;

// basic handler that responds with a static string
#[axum::debug_handler]
pub async fn dashboard() -> impl IntoResponse {
    let next_arrivals = match transportation::get_next_arrivals().await {
        Ok(data) => data,
        Err(e) => {
            println!("Error: {:?}", e);
            vec![]
        }
    };

    let residents = get_residents();

    let weather = get_weather_data().await.expect("Couldn't get weather data");

    let template = DashboardTemplate {
        housing_company: HousingCompany {
            name: "Portus Novus".to_string(),
            address: "Capellanranta 3 D".to_string(),
        },
        next_arrivals,
        residents,
        weather
    };

    println!("Props: {:?}", template);

    HtmlTemplate(template)
}
