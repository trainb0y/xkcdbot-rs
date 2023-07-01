use datetime::ISO;
use poise::builtins::register_in_guild;
use poise::serenity_prelude::{Button, ButtonStyle};
use poise::{serenity_prelude as serenity, Command};

use crate::comic::Comic;

mod comic;
mod command;

pub struct Data {
    client: reqwest::Client,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const EXPLAIN_URL: &str = "https://explainxkcd.com";
const XKCD_URL: &str = "https://xkcd.com";

// save me from myself
fn commands() -> Vec<Command<Data, Error>> {
    vec![
        command::ping(),
        command::xkcd(),
        command::help(),
        command::about(),
    ]
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .token(std::env::var("TOKEN").expect("missing TOKEN env var"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                // run when bot connects to discord
                if let Ok(guild_id) = std::env::var("TEST_GUILD") {
                    // add commands to the guild as guild-specific commands to get them to propagate faster
                    // waiting for discord is cringe
                    let guild = serenity::GuildId::from(guild_id.parse::<u64>().unwrap());

                    println!("debug commands guild: {}", guild_id);
                    register_in_guild(ctx, &commands(), guild).await?;
                };

                Ok(Data {
                    client: reqwest::Client::new(),
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands: commands(),
            ..Default::default()
        });

    framework.run().await.unwrap();
}

async fn send_comic_embed(ctx: Context<'_>, comic: &Comic) {
    ctx.send(|rep| {
        rep.components(|comp| {
            comp.create_action_row(|row| {
                row.create_button(|button| {
                    button
                        .label("Explain")
                        .style(ButtonStyle::Link)
                        .url(comic.get_explain_link())
                });
                row.create_button(|button| {
                    button
                        .label("◀️")
                        .style(ButtonStyle::Primary)
                        .custom_id("back".to_string())
                });
                row.create_button(|button| {
                    button
                        .label("🎲")
                        .style(ButtonStyle::Primary)
                        .custom_id("random".to_string())
                });
                row.create_button(|button| {
                    button
                        .label("▶️")
                        .style(ButtonStyle::Primary)
                        .custom_id("next".to_string())
                });
                row
            })
        });
        rep.embed(|embed| {
            embed.title(&comic.title);
            embed.description(format!(
                "`#{}` - {} - [see on xkcd.com]({})",
                &comic.num,
                comic.get_date().iso().to_string(),
                comic.get_comic_link()
            ));
            embed.image(&comic.img);
            embed.footer(|footer| {
                footer.text(&comic.alt);
                footer
            })
        })
    })
    .await
    .expect("idek anymore");
}
