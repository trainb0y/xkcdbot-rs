use datetime::ISO;
use rand::Rng;

use crate::comic::Comic;
use crate::{Context, Error};

/// Is the bot even alive?
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("Pong!").await?;
    Ok(())
}

/// Help and documentation
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.send(|rep| {
        rep.embed(|embed| {
            embed.title("xkcd bot help");
            embed.description("I cannot help you");
            embed
        });
        rep
    })
    .await?;
    Ok(())
}

/// Bug reporting, source code, and more
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("todo").await?;
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
    let comic = Comic::get_num(&ctx.data().client, num).await.unwrap();
    comic.send_comic_embed(ctx).await;
    Ok(())
}

/// Get the latest comic
#[poise::command(slash_command)]
async fn latest(ctx: Context<'_>) -> Result<(), Error> {
    let comic = Comic::get_latest(&ctx.data().client).await.unwrap();
    comic.send_comic_embed(ctx).await;
    Ok(())
}

/// Get a random comic
#[poise::command(slash_command)]
async fn random(ctx: Context<'_>) -> Result<(), Error> {
    let latest = Comic::get_latest(&ctx.data().client).await.unwrap();
    let num = rand::thread_rng().gen_range(1..(latest.num + 1));
    let comic = Comic::get_num(&ctx.data().client, num).await.unwrap();
    comic.send_comic_embed(ctx).await;
    Ok(())
}
