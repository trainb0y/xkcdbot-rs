use poise::samples::register_in_guild;
use poise::serenity_prelude::GuildId;
use poise::{serenity_prelude as serenity, Command, FrameworkBuilder};

#[derive(Copy, Clone)]
struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// save me from myself
fn commands() -> Vec<Command<Data, Error>> {
    return vec![ping()];
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

                Ok(Data {})
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
