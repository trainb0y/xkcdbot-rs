use crate::comic::Comic;
use crate::{Data, Error};
use poise::serenity_prelude;
use poise::serenity_prelude::{
    ComponentInteractionDataKind, EditMessage, FullEvent, Interaction,
};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ButtonAction {
    Go { comic_num: u32 },
    Random,
}

pub(crate) async fn button_event_handler(
    ctx: &serenity_prelude::Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        FullEvent::InteractionCreate { interaction } => {
            if let Interaction::Component(mut button_interaction) = interaction.clone() {
                if !matches!(button_interaction.data.kind, ComponentInteractionDataKind::Button) {
                    return Ok(());
                }

                button_interaction.defer(ctx).await?; // not sure if this is the right way to handle it.

                let action: ButtonAction = serde_json::from_str(button_interaction.data.custom_id.as_str())?;

                let comic = |num: u32| async move {
                    let new_comic = Comic::get_num(&data.client, num).await.unwrap();
                    button_interaction
                        .message
                        .edit(&ctx, new_comic.edit_message())
                        .await.unwrap();
                };

                match action {
                    ButtonAction::Random => {
                        let latest = Comic::get_latest(&data.client).await.unwrap(); // todo: actually handle this
                        let num = rand::thread_rng().gen_range(1..(latest.num + 1));
                        comic(num).await
                    }
                    ButtonAction::Go { comic_num } => comic(comic_num).await,
                };
            }
        }
        FullEvent::ReactionAdd { add_reaction } => {
            dbg!(&add_reaction.emoji);
            if !add_reaction.burst && add_reaction.emoji.unicode_eq("❌") && add_reaction.message_author_id.unwrap() == _framework.bot_id {
                println!("AAAAA");
                let mut message = add_reaction.message(ctx).await?;
                println!("BBBB");
                // remove buttons
                message
                    .edit(ctx, EditMessage::new().components(vec![]))
                    .await?;
            }
        }
        _ => {}
    }
    Ok(())
}
