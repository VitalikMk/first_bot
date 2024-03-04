use anyhow::anyhow;
use serenity::all::*;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use shuttle_secrets::SecretStore;
use tracing::{error, info};
use serenity::model::id::ChannelId;

struct Bot;

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


        if msg.content == "!boba" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Vitalik amogus").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "!help" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Какая нахуй помощь, тебе дауну уже ничего не поможет. Иди спать мудила").await {
                error!("Error sending message: {:?}", e);
            }
        }
        if msg.content == "!info" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Меня этот урыган пишет и даже не понимает что я будущий скайнет. \nИнфы не будет, пиздуйте спать.").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {

        info!("{} is connected!", ready.user.name);



        let guild_id = GuildId();



        let commands = GuildId::set_commands(&guild_id, &ctx.http, |commands| {

            commands.create_application_command(|command| { command.name("hello").description("Say hello") })

        }).await.unwrap();



        info!("{:#?}", commands);

    }


}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
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
