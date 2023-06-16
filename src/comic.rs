use datetime::{LocalDate, Month};
use serde::Deserialize;

use crate::{EXPLAIN_URL, XKCD_URL};

#[derive(Deserialize)]
pub struct Comic {
    pub num: u32,
    pub title: String,
    pub alt: String,

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

    pub async fn get(
        num: u32,
        client: &reqwest::Client,
    ) -> Result<Comic, Box<dyn std::error::Error>> {
        Ok(client
            .get(format!("{}/{}/info.0.json", XKCD_URL, num))
            .send()
            .await?
            .json::<Comic>()
            .await?)
    }
}
