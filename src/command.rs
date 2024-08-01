use poise::serenity_prelude::{ButtonStyle, CreateActionRow, CreateButton, CreateEmbed};
use poise::CreateReply;
use rand::Rng;

use crate::comic::Comic;
use crate::{Context, Error};

/// Help and documentation
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.send(
        CreateReply::default().embed(
            CreateEmbed::new()
                .title("xkcd bot help")
                .field(
                    "Commands",
                    "
                `/help`         - Display this message
                `/about`        - Display uptime, version, and links
                `/xkcd random`  - Get a random xkcd
                `/xkcd get <n>` - Get a comic by number
                ",
                    false,
                )
                .field(
                    "Tips",
                    "
                React with :x: to remove buttons
                ",
                    false,
                ),
        ),
    )
    .await?;
    Ok(())
}

/// Bug reporting, source code, and more
#[poise::command(slash_command)]
pub async fn about(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer_ephemeral().await?;
    ctx.send(
        CreateReply::default()
            .embed(
                CreateEmbed::new()
                    .title("About xkcd bot")
                    .field("Version", env!("CARGO_PKG_VERSION"), true)
                    .field(
                        "Uptime",
                        format!("<t:{}:R>", ctx.data().start_time.timestamp()),
                        true,
                    )
                    .field(
                        "About",
                        "xkcd bot - a bot for Randall Munroe's [xkcd](https://xkcd.com)
                (Comics under [CC BY-NC 2.5](https://creativecommons.org/licenses/by-nc/2.5/))",
                        false,
                    ),
            )
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new_link(env!("CARGO_PKG_REPOSITORY"))
                    .label("Source Code")
                    .style(ButtonStyle::Secondary),
                CreateButton::new_link(format!("{}/issues", env!("CARGO_PKG_REPOSITORY")))
                    .label("Report an Issue")
                    .style(ButtonStyle::Secondary),
            ])]),
    )
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
