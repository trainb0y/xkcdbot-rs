use chrono::{DateTime, Utc};
use poise::builtins::register_globally;
use poise::builtins::register_in_guild;
use poise::serenity_prelude::{CacheHttp, EventHandler};
use poise::{serenity_prelude as serenity, Command};

mod buttons;
mod comic;
mod command;

pub struct Data {
    client: reqwest::Client,
    start_time: DateTime<Utc>,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

const EXPLAIN_URL: &str = "https://explainxkcd.com";
const XKCD_URL: &str = "https://xkcd.com";

// save me from myself
fn commands() -> Vec<Command<Data, Error>> {
    vec![command::xkcd(), command::help(), command::about()]
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands(),
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(buttons::button_event_handler(
                    _ctx, event, _framework, _data,
                ))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, _framework| {
            Box::pin(async move {
                // run when bot connects to discord
                if let Ok(guild_id) = std::env::var("TEST_GUILD") {
                    // add commands to the guild as guild-specific commands to get them to propagate faster
                    // waiting for discord is cringe
                    let guild = serenity::GuildId::from(guild_id.parse::<u64>().unwrap());

                    println!("debug commands guild: {}", guild_id);
                    register_in_guild(ctx, &commands(), guild).await?;
                };

                register_globally(ctx, &commands()).await.unwrap();

                Ok(Data {
                    client: reqwest::Client::new(),
                    start_time: Utc::now(),
                })
            })
        })
        .build();

    let mut client = serenity::ClientBuilder::new(
        std::env::var("TOKEN").expect("missing TOKEN env var"),
        serenity::GatewayIntents::non_privileged(),
    )
        .framework(framework)
        .await
        .unwrap();

    client.start().await.unwrap()
}
