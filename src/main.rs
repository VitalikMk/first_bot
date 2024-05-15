use std::time::Duration;
use anyhow::anyhow;
use serenity::all::*;
use serenity::async_trait;
use serenity::model::channel::Message;

use shuttle_secrets::SecretStore;
use tracing::{error};
use serde_json::read::Read;

use serenity::futures::StreamExt;

use std::iter::Iterator;
use std::str::pattern::Searcher;


struct Bot;

fn sound_button(name: &str, emoji: ReactionType) -> CreateButton {
    // To add an emoji to buttons, use .emoji(). The method accepts anything ReactionType or
    // anything that can be converted to it. For a list of that, search Trait Implementations in
    // the docs for From<...>.
    CreateButton::new(name).emoji(emoji)
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {

        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "word").await {
                error!("Error sending message: {:?}", e);
            }
        }


        if msg.content == "!ping" {

            let channel = match msg.channel_id.to_channel(&ctx).await {
                Ok(channel) => channel,
                Err(why) => {
                    println!("Error getting channel: {why:?}");

                    return;
                },
            };


            let response = MessageBuilder::new()
                .push("User ")
                .push_bold_safe(&msg.author.name)
                .push(" used the 'ping' command in the ")
                .mention(&channel)
                .push(" channel")
                .build();

            if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
                println!("Error sending message: {why:?}");
            }
        }


        if msg.content == "!messageme" {
            let builder = CreateMessage::new().content("Hello!");
            let dm = msg.author.dm(&ctx, builder).await;

            if let Err(why) = dm {
                println!("Error when direct messaging user: {why:?}");
            }
        }


        if msg.content == "!vitalik" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "vitalik the best").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "!help" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Write to admin Gwar").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "!info" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Test info. This is my first bot on rust and my project.").await {
                error!("Error sending message: {:?}", e);
            }
        }

        if msg.content != "animal" {
            return;
        }
        

        // Ask the user for its favorite animal
        let m = msg
            .channel_id
            .send_message(
                &ctx,
                CreateMessage::new().content("Please select your favorite animal").select_menu(
                    CreateSelectMenu::new("animal_select", CreateSelectMenuKind::String {
                        options: vec![
                            CreateSelectMenuOption::new("🐈 meow", "Cat"),
                            CreateSelectMenuOption::new("🐕 woof", "Dog"),
                            CreateSelectMenuOption::new("🐎 neigh", "Horse"),
                            CreateSelectMenuOption::new("🦙 hoooooooonk", "Alpaca"),
                            CreateSelectMenuOption::new("🦀 crab rave", "Ferris"),
                        ],
                    })
                        .custom_id("animal_select")
                        .placeholder("No animal selected"),
                ),
            )
            .await
            .unwrap();

        // Wait for the user to make a selection
        // This uses a collector to wait for an incoming event without needing to listen for it
        // manually in the EventHandler.
        let interaction = match m
            .await_component_interaction(&ctx.shard)
            .timeout(Duration::from_secs(60 * 3))
            .await
        {
            Some(x) => x,
            None => {
                m.reply(&ctx, "Timed out").await.unwrap();
                return;
            },
        };

        // data.values contains the selected value from each select menus. We only have one menu,
        // so we retrieve the first
        let animal = match &interaction.data.kind {
            ComponentInteractionDataKind::StringSelect {
                values,
            } => &values[0],
            _ => panic!("unexpected interaction data kind"),
        };

        // Acknowledge the interaction and edit the message
        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::UpdateMessage(
                    CreateInteractionResponseMessage::default()
                        .content(format!("You chose: **{animal}**\nNow choose a sound!"))
                        .button(sound_button("meow", "🐈".parse().unwrap()))
                        .button(sound_button("woof", "🐕".parse().unwrap()))
                        .button(sound_button("neigh", "🐎".parse().unwrap()))
                        .button(sound_button("hoooooooonk", "🦙".parse().unwrap()))
                        .button(sound_button(
                            "crab rave",
                            // Custom emojis in Discord are represented with
                            // `<:EMOJI_NAME:EMOJI_ID>`. You can see this by posting an emoji in
                            // your server and putting a backslash before the emoji.
                            //
                            // Because ReactionType implements FromStr, we can use .parse() to
                            // convert the textual emoji representation to ReactionType
                            "<:ferris:381919740114763787>".parse().unwrap(),
                        )),
                ),
            )
            .await
            .unwrap();

        let mut interaction_stream =
            m.await_component_interaction(&ctx.shard).timeout(Duration::from_secs(60 * 3)).stream();

        while let Some(interaction) = interaction_stream.next().await {
            let sound = &interaction.data.custom_id;
            // Acknowledge the interaction and send a reply
            interaction
                .create_response(
                    &ctx,
                    // This time we dont edit the message but reply to it
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::default()
                            // Make the message hidden for other users by setting `ephemeral(true)`.
                            .ephemeral(true)
                            .content(format!("The **{animal}** says __{sound}__")),
                    ),
                )
                .await
                .unwrap();
        }

        // Delete the orig message or there will be dangling components (components that still
        // exist, but no collector is running so any user who presses them sees an error)
        m.delete(&ctx).await.unwrap()


    }





    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected", ready.user.name );
    }




}

#[shuttle_runtime::main]
async fn serenity(#[shuttle_secrets::Secrets] secret_store: SecretStore, ) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}

