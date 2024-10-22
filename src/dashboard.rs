use axum::response::IntoResponse;
use residents::get_residents;
use templates::{DashboardTemplate, HousingCompany, HtmlTemplate};
use tokio::task;
use weather::{get_weather_data, EMPTY_WEATHER_DATA};

use super::*;

// basic handler that responds with a static string
#[axum::debug_handler]
pub async fn dashboard() -> impl IntoResponse {
    // residents worker
    let handle_residents = task::spawn(async {
        let residents = get_residents();
        residents
    });

    // weather worker
    let handle_weather = task::spawn(async {
        let weather = match get_weather_data().await {
            Ok(data) => data,
            Err(e) => {
                println!("Error: {:?}", e);
                EMPTY_WEATHER_DATA
            }
        };
        weather
    });

    // transportation worker
    let handle_transportation = task::spawn(async {
        let next_arrivals = match transportation::get_next_arrivals().await {
            Ok(data) => data,
            Err(e) => {
                println!("Error: {:?}", e);
                vec![]
            }
        };

        next_arrivals
    });

    let (residents, next_arrivals, weather) =
        tokio::join!(handle_residents, handle_transportation, handle_weather);

    HtmlTemplate(DashboardTemplate {
        housing_company: HousingCompany {
            name: "Portus Novus".to_string(),
            address: "Capellanranta 3 D".to_string(),
        },
        next_arrivals: match next_arrivals {
            Ok(data) => data,
            Err(e) => {
                println!("Error: {:?}", e);
                vec![]
            }
        },
        residents: match residents {
            Ok(data) => data,
            Err(e) => {
                println!("Error: {:?}", e);
                vec![]
            }
        },
        weather: match weather {
            Ok(data) => data,
            Err(e) => {
                println!("Error: {:?}", e);
                EMPTY_WEATHER_DATA
            }
        },
    })
}
