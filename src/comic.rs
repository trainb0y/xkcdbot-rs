use datetime::{LocalDate, Month, ISO};
use poise::serenity_prelude::{ButtonStyle, CreateComponents, CreateEmbed, EditMessage};
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

    fn create_buttons(&self, comp: &mut CreateComponents) {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Explain")
                    .style(ButtonStyle::Link)
                    .url(self.get_explain_link())
            });
            row.create_button(|button| {
                button.label("◀️").style(ButtonStyle::Primary).custom_id(
                    serde_json::to_string(&ButtonAction::Go {
                        comic_num: self.num - 1,
                    })
                    .unwrap(),
                )
            });
            row.create_button(|button| {
                button
                    .label("🎲")
                    .style(ButtonStyle::Primary)
                    .custom_id(serde_json::to_string(&ButtonAction::Random).unwrap())
            });
            row.create_button(|button| {
                button.label("▶️").style(ButtonStyle::Primary).custom_id(
                    serde_json::to_string(&ButtonAction::Go {
                        comic_num: self.num + 1,
                    })
                    .unwrap(),
                )
            });
            row
        });
    }

    fn create_embed(&self, embed: &mut CreateEmbed) {
        embed.title(&self.title);
        embed.description(format!(
            "`#{}` - {} - [see on xkcd.com]({})",
            &self.num,
            self.get_date().iso(),
            self.get_comic_link()
        ));
        embed.image(&self.img);
        embed.footer(|footer| {
            footer.text(&self.alt);
            footer
        });
    }

    pub async fn send_comic_embed(&self, ctx: Context<'_>) {
        ctx.send(|rep| {
            rep.components(|comp| {
                self.create_buttons(comp);
                comp
            });
            rep.embed(|embed| {
                self.create_embed(embed);
                embed
            })
        })
        .await
        .unwrap();
    }

    pub fn edit_in_message(&self, message: &mut EditMessage<'_>) {
        message.components(|comp| {
            self.create_buttons(comp);
            comp
        });
        message.embed(|embed| {
            self.create_embed(embed);
            embed
        });
    }
}
