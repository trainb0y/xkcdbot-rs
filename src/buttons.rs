use crate::comic::Comic;
use crate::{Data, Error};
use poise::serenity_prelude::{ComponentType, Interaction};
use poise::{serenity_prelude, Event};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ButtonAction {
    Go { comic_num: u32 },
    Random,
}

pub async fn button_event_handler(
    ctx: &serenity_prelude::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let Event::InteractionCreate { interaction } = event {
        if let Interaction::MessageComponent(mut button) = interaction.clone() {
            if button.data.component_type != ComponentType::Button {
                return Ok(());
            }

            button.defer(ctx).await?; // not sure if this is the right way to handle it.

            let action: ButtonAction = serde_json::from_str(button.data.custom_id.as_str())?;

            let comic = |num: u32| async move {
                let new_comic = Comic::get_num(&data.client, num).await.unwrap();
                button
                    .message
                    .edit(&ctx, |message| {
                        new_comic.edit_in_message(message);
                        message
                    })
                    .await
                    .unwrap();
            };

            match action {
                ButtonAction::Random => {
                    let latest = Comic::get_latest(&data.client).await.unwrap(); // todo: actually handle this
                    let num = rand::thread_rng().gen_range(1..(latest.num + 1));
                    comic(num).await
                }
                ButtonAction::Go { comic_num } => comic(comic_num).await
                
            };
        }
    }
    Ok(())
}
