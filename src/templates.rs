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

#[derive(askama::Template, Debug)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate {
    pub housing_company: HousingCompany,
    pub next_arrivals: Vec<Arrivals>,
    pub residents: Vec<Residents>,
    pub weather: WeatherData,
    pub news: Vec<NewsItem>,
}
