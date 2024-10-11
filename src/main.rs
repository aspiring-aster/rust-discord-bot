use rust_discord_bot::*;
use serenity::all::{CreateEmbed, CreateEmbedFooter, CreateMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::{async_trait, gateway};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.guild_id != None {
            return;
        }
        match msg.content.as_str() {
            "clear" => {
                clear_messages(ctx, msg).await;
            }
            "quiz" => {
                let footer = CreateEmbedFooter::new("Bot of '09 Developed by @anon for Class of '09").icon_url("https://images-ext-1.discordapp.net/external/AJhyvuDbJn-xEOg-8aZWAUtsg7Z9OGkWuSOminCc0hk/%3Fsize%3D256/https/cdn.discordapp.com/icons/1129581515584589914/162d2a7c4059aa53aba76bba0702788e.webp");
                let embed = CreateEmbed::new()
                    .title("Quiz Results")
                    .footer(footer)
                    .description("# You Have Successfully Completed The Quiz!")
                    .image("https://i.imgur.com/cdSerzD.png")
                    .thumbnail("https://i.imgur.com/cfXguCv.png");
                let new_m = CreateMessage::new().add_embed(embed);
                msg.channel_id.send_message(&ctx.http, new_m).await.ok();
            }
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    const TOKEN: &str = "";
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let activity_data = gateway::ActivityData::playing("Class of `09");
    let mut client = Client::builder(&TOKEN, intents)
        .event_handler(Handler)
        .activity(activity_data)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
