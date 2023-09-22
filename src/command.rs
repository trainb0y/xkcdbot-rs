use poise::serenity_prelude::ButtonStyle;
use rand::Rng;

use crate::comic::Comic;
use crate::{Context, Error};

/// Help and documentation
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.send(|rep| {
        rep.embed(|embed| {
            embed.title("xkcd bot help");
            embed.field(
                "Commands",
                "
            `/help`         - Display this message
            `/about`        - Display uptime, version, and links
            `/xkcd random`  - Get a random xkcd
            `/xkcd get <n>` - Get a comic by number
            ",
                false,
            );
            embed.field(
                "Tips",
                "
            React with :x: to remove buttons
            
            ",
                false,
            );
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
    ctx.defer_ephemeral().await?;
    ctx.send(|rep| {
        rep.embed(|embed| {
            embed.title("About xkcd bot");
            embed.field("Version", env!("CARGO_PKG_VERSION"), true);
            embed.field("Uptime", "forever", true);
            embed.field(
                "About",
                "xkcd bot - a bot for Randall Munroe's [xkcd](https://xkcd.com)\
                Comics under [CC BY-NC 2.5](https://creativecommons.org/licenses/by-nc/2.5/)",
                false,
            );
            embed
        });
        rep.components(|components| {
            components.create_action_row(|row| {
                row.create_button(|button| {
                    button.label("Source Code");
                    button.style(ButtonStyle::Link);
                    button.url(env!("CARGO_PKG_REPOSITORY"));
                    button
                });
                row.create_button(|button| {
                    button.label("Report an Issue");
                    button.style(ButtonStyle::Link);
                    button.url(format!("{}/issues", env!("CARGO_PKG_REPOSITORY")));
                    button
                });
                row
            });
            components
        });
        rep
    })
    .await?;
    Ok(())
}

/// xkcd related commands
#[poise::command(
    slash_command,
    required_permissions = "EMBED_LINKS",
    subcommands("get", "latest", "random")
)]
pub async fn xkcd(_: Context<'_>) -> Result<(), Error> {
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
