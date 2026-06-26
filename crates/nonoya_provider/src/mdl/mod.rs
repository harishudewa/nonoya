use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

pub struct MdlProvider;

#[derive(Debug, Default)]
pub struct MdlGetSchedule {
    pub day: String,
    pub date: String,
    pub full_date: String,
    pub contents: Vec<MdlContent>,
}

#[derive(Debug, Default)]
pub struct MdlContent {
    pub title: String,
    pub img: String,
}

impl MdlProvider {
    pub async fn get_schedule() -> Result<Vec<MdlGetSchedule>, String> {
        let endpoint = "https://mydramalist.com/episode-calendar?view=small&scope=all&tz=Asia%2FJakarta&country%5B%5D=South+Korea";
        let client = reqwest::Client::new();
        let res = client
            .get(endpoint)
            .header(USER_AGENT, "mac")
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let document = Html::parse_document(&res);
        let days = Selector::parse(r#"div .day_of_week .full"#).unwrap();
        let dates = Selector::parse(r#"div .day_of_month"#).unwrap();

        let days: Vec<String> = document.select(&days).map(|v| v.inner_html()).collect();
        let dates: Vec<String> = document.select(&dates).map(|v| v.inner_html()).collect();

        let mut schedules = Vec::with_capacity(days.len());

        for (i, d) in dates.iter().enumerate() {
            let full_date_selector = Selector::parse(&format!("div#d{} small", d)).unwrap();
            let full_date: Vec<String> = document
                .select(&full_date_selector)
                .map(|v| v.inner_html())
                .collect();
            let full_date = full_date.get(0).map(|v| v.to_string()).unwrap_or_default();

            let mut schedule = MdlGetSchedule {
                day: days.get(i).map(|v| v.to_string()).unwrap_or_default(),
                date: d.to_string(),
                full_date,
                ..Default::default()
            };

            let title_selector =
                Selector::parse(&format!("div#d{} div .episode-card-sm .top-append + a", d))
                    .unwrap();
            let cover_selector = Selector::parse(&format!("div#d{} div .cover-sm img", d)).unwrap();

            let title_list: Vec<String> = document
                .select(&title_selector)
                .map(|v| v.inner_html())
                .collect();

            let img_list: Vec<String> = document
                .select(&cover_selector)
                .map(|v| {
                    v.attr("src")
                        .unwrap_or_else(|| v.attr("data-src").unwrap_or(""))
                        .to_string()
                        .replace("4c.jpg", "4t.jpg")
                })
                .collect();

            let contents: Vec<MdlContent> = title_list
                .into_iter()
                .zip(img_list)
                .map(|v| MdlContent {
                    title: v.0,
                    img: v.1,
                })
                .collect();

            schedule.contents = contents;
            schedules.push(schedule);
        }

        println!("schedules: {:#?}", schedules);

        Ok(schedules)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_schedule() {
        MdlProvider::get_schedule().await.unwrap();
        assert_eq!(true, true)
    }
}
