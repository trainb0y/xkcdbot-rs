use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use poise::serenity_prelude::{
    ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateEmbedFooter, CreateMessage,
    EditMessage,
};
use poise::CreateReply;
use serde::Deserialize;

use crate::buttons::ButtonAction;
use crate::{Context, EXPLAIN_URL, XKCD_URL};

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

    pub fn get_date(&self) -> NaiveDate {
        // this is cringe and should get cleaned up
        NaiveDate::from_ymd_opt(
            self.year.parse::<i32>().unwrap(),
            self.month.parse::<u32>().unwrap(),
            self.day.parse::<u32>().unwrap(),
        )
            .unwrap()
    }

    async fn get(
        client: &reqwest::Client,
        json_url: &str,
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
        Comic::get(client, format!("{}/{}", XKCD_URL, num).as_str()).await
    }

    pub async fn get_latest(client: &reqwest::Client) -> Result<Comic, Box<dyn std::error::Error>> {
        Comic::get(client, XKCD_URL).await
    }

    fn create_actions(&self) -> CreateActionRow {
        CreateActionRow::Buttons(vec![
            CreateButton::new_link(self.get_explain_link())
                .label("Explain")
                .style(ButtonStyle::Secondary),
            CreateButton::new(
                serde_json::to_string(&ButtonAction::Go {
                    comic_num: self.num - 1,
                })
                    .unwrap(),
            )
                .label("◀️")
                .style(ButtonStyle::Primary),
            CreateButton::new(serde_json::to_string(&ButtonAction::Random).unwrap())
                .label("🎲")
                .style(ButtonStyle::Primary),
            CreateButton::new(
                serde_json::to_string(&ButtonAction::Go {
                    comic_num: self.num + 1,
                })
                    .unwrap(),
            )
                .label("▶️")
                .style(ButtonStyle::Primary),
        ])
    }

    fn create_embed(&self) -> CreateEmbed {
        CreateEmbed::new()
            .title(&self.title)
            .description(format!(
                "`#{}` - {} - [see on xkcd.com]({})",
                &self.num,
                self.get_date(),
                self.get_comic_link()
            ))
            .image(&self.img)
            .footer(CreateEmbedFooter::new(&self.alt))
    }

    pub async fn send_comic_embed(&self, ctx: Context<'_>) {
        ctx.send(
            CreateReply::default()
                .components(vec![self.create_actions()])
                .embed(self.create_embed()),
        )
            .await
            .unwrap();
    }

    pub fn edit_message(&self) -> EditMessage {
        EditMessage::new()
            .components(vec![self.create_actions()])
            .embed(self.create_embed())
    }
}
