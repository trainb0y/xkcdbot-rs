use datetime::{LocalDate, Month};
use serde::Deserialize;

use crate::{EXPLAIN_URL, XKCD_URL};

#[derive(Deserialize)]
pub struct Comic {
    pub num: u32,
    pub title: String,
    pub alt: String,
    pub img: String,

    // why is it like this?
    // I don't know, ask Randall
    day: String,
    month: String,
    year: String,
}

impl Comic {
    pub fn get_comic_link(&self) -> String {
        format!("{}/{}", XKCD_URL, self.num)
    }

    pub fn get_explain_link(&self) -> String {
        format!("{}/{}", EXPLAIN_URL, self.num)
    }

    pub fn get_date(&self) -> LocalDate {
        // this is cringe and should get cleaned up
        LocalDate::ymd(
            self.year.parse::<i64>().unwrap(),
            Month::from_one(self.month.parse::<i8>().unwrap()).unwrap(),
            self.day.parse::<i8>().unwrap(),
        )
        .unwrap()
    }

    async fn get(
        client: &reqwest::Client,
        json_url: String,
    ) -> Result<Comic, Box<dyn std::error::Error>> {
        Ok(client
            .get(format!("{}/info.0.json", json_url))
            .send()
            .await?
            .json::<Comic>()
            .await?)
    }

    pub async fn get_num(
        client: &reqwest::Client,
        num: u32,
    ) -> Result<Comic, Box<dyn std::error::Error>> {
        Comic::get(client, format!("{}/{}", XKCD_URL, num)).await
    }

    pub async fn get_latest(client: &reqwest::Client) -> Result<Comic, Box<dyn std::error::Error>> {
        Comic::get(client, XKCD_URL.into()).await
    }
}
