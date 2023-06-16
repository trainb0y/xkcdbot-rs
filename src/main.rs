use datetime::{LocalDate, Month};
use poise::futures_util::TryFutureExt;
use poise::samples::register_in_guild;
use poise::serenity_prelude::GuildId;
use poise::{serenity_prelude as serenity, Command, FrameworkBuilder};
use serde::Deserialize;

struct Data {
    client: reqwest::Client,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const EXPLAIN_URL: &str = "https://explainxkcd.com";
const XKCD_URL: &str = "https://xkcd.com";

// save me from myself
fn commands() -> Vec<Command<Data, Error>> {
    vec![ping(), debug_get()]
}

#[derive(Deserialize)]
struct Comic {
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
    fn get_comic_link(&self) -> String {
        format!("{}/{}", XKCD_URL, self.num)
    }

    fn get_explain_link(&self) -> String {
        format!("{}/{}", EXPLAIN_URL, self.num)
    }

    fn get_date(&self) -> LocalDate {
        // this is cringe and should get cleaned up
        LocalDate::ymd(
            self.year.parse::<i64>().unwrap(),
            Month::from_one(self.month.parse::<i8>().unwrap()).unwrap(),
            self.day.parse::<i8>().unwrap(),
        )
        .unwrap()
    }

    async fn get(num: u32, client: &reqwest::Client) -> Result<Comic, Box<dyn std::error::Error>> {
        Ok(client
            .get(format!("{}/{}/info.0.json", XKCD_URL, num))
            .send()
            .await?
            .json::<Comic>()
            .await?)
    }
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
                    let guild = GuildId::from(guild_id.parse::<u64>().unwrap());

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

/// Is the bot alive?
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// reeee
#[poise::command(slash_command)]
async fn debug_get(ctx: Context<'_>, num: u32) -> Result<(), Error> {
    let comic = Comic::get(num, &ctx.data().client).await.unwrap();
    ctx.say(format!("Got comic! Title: {}", comic.title))
        .await?;
    Ok(())
}
