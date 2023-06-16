use poise::{serenity_prelude as serenity, Command};

use crate::comic::Comic;
use crate::{Context, Error};

/// Is the bot alive?
#[poise::command(slash_command)]
async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// xkcd related commands
#[poise::command(slash_command, subcommands("get"))]
async fn xkcd(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Get a comic by its number
#[poise::command(slash_command)]
async fn get(ctx: Context<'_>, num: u32) -> Result<(), Error> {
    let comic = Comic::get(num, &ctx.data().client).await.unwrap();
    ctx.say(format!("Got comic! Title: {}", comic.title))
        .await?;
    Ok(())
}
