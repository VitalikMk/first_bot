use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
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
            if let Err(e) = msg.channel_id.say(&ctx.http, "Меня этот урыган пишет и даже не понимает что я будущий скайнет. \n Инфы не будет, пиздуйте спать.").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
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
