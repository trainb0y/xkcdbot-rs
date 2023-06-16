use datetime::fmt::DateFormat;
use datetime::ISO;
use poise::serenity_prelude::CreateEmbed;
use poise::{serenity_prelude as serenity, Command};

use crate::comic::Comic;
use crate::{Context, Error};

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
    subcommands("get", "latest")
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
    let comic = Comic::get_latest(&ctx.data().client).await.unwrap(); // todo: actually handle
    send_comic_embed(ctx, &comic).await;
    Ok(())
}

async fn send_comic_embed(ctx: Context<'_>, comic: &Comic) {
    ctx.send(|rep| {
        rep.embed(|embed| {
            embed.title(&comic.title);
            embed.description(format!(
                "`#{}` - {}",
                &comic.num,
                comic.get_date().iso().to_string()
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
