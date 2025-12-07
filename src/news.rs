#[derive(Clone, Debug)]
pub struct NewsItem {
  pub external_link: Option<String>,
  pub heading: String,
  pub content: String,
  pub of_type: String
}

pub async fn load_news() -> Vec<NewsItem> {

  let news: &[NewsItem] = &[
    NewsItem {
      external_link: Some(String::from("https://example.com/news1")),
      heading: String::from("News Heading 1"),
      content: String::from("This is the **content** of the first news item."),
      of_type: String::from("info"),
    },
    NewsItem {
      external_link: Some(String::from("https://example.com/news2")),
      heading: String::from("Elevator Maintenance"),
      content: String::from("The elevator will be out of service on Monday for maintenance."),
      of_type: String::from("alert"),
    },
    NewsItem {
      external_link: Some(String::from("https://example.com/news2")),
      heading: String::from("Board Meeting"),
      content: String::from("A board meeting was held 1st of january. The full newsletter is available in the portal."),
      of_type: String::from("board"),
    },
  ];

  news.to_vec()
}