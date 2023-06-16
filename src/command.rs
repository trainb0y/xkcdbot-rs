use datetime::fmt::DateFormat;
use datetime::ISO;
use poise::serenity_prelude::CreateEmbed;
use poise::{serenity_prelude as serenity, Command};
use rand::Rng;

use crate::comic::Comic;
use crate::{send_comic_embed, Context, Error};

/// Is the bot alive?
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// xkcd related commands
#[poise::command(
    slash_command,
    required_permissions = "EMBED_LINKS",
    subcommands("get", "latest", "random")
)]
pub async fn xkcd(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Get a comic by its number
#[poise::command(slash_command)]
async fn get(ctx: Context<'_>, num: u32) -> Result<(), Error> {
    let comic = Comic::get_num(&ctx.data().client, num).await.unwrap(); // todo: actually handle
    send_comic_embed(ctx, &comic).await;
    Ok(())
}

/// Get the latest comic
#[poise::command(slash_command)]
async fn latest(ctx: Context<'_>) -> Result<(), Error> {
    let comic = Comic::get_latest(&ctx.data().client).await.unwrap(); // todo: handle the case
    send_comic_embed(ctx, &comic).await;
    Ok(())
}

/// Get a random comic
#[poise::command(slash_command)]
async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let latest = Comic::get_latest(&ctx.data().client).await.unwrap(); // todo: actually handle this
    let num = rand::thread_rng().gen_range(1..(latest.num + 1));
    let comic = Comic::get_num(&ctx.data().client, num).await.unwrap(); // todo: you guessed it, handle
    send_comic_embed(ctx, &comic).await;
    Ok(())
}
