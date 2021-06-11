use async_std::task;
use serenity::client::Context;
use serenity::gateway::Shard;
use serenity::model::gateway::Activity;
use serenity::model::id::ChannelId;
use serenity::model::user::OnlineStatus;
use serenity::prelude::*;
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
    },
};
use std::io::Read;
use std::io::Write;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        loop {
            let caps = DesiredCapabilities::firefox();
            let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps)
                .await
                .unwrap();

            // Navigate to URL.
            driver
                .get("https://www.epicgames.com/store/en-US/free-games")
                .await;

            // Navigate to page, by chaining futures together and awaiting the result.
            let elem = driver
                .find_element(By::XPath("//span[contains(.,\"Free Now\")]"))
                .await
                .unwrap();

            let x = elem
                .find_element(By::XPath("//span[@data-testid=\"offer-title-info-title\"]"))
                .await
                .unwrap();

            let text = x.text().await.unwrap();

            let link =
                elem.find_element(By::XPath("//span[@data-testid=\"offer-title-info-title\"]"));

            driver.quit().await;

            let channel_to = ChannelId(745229746727551020);
            let _ = ctx
                .set_presence(
                    Some(Activity::playing("Written in rust")),
                    OnlineStatus::Online,
                )
                .await;

            let y = channel_to.say(ctx.http, newtext + link).await;
            task::sleep(Duration::from_secs(3600)).await;
            println!("restarting");
        }
    }
}

fn save_name(name: &str) {
    let mut file = std::fs::File::create("data.txt").expect("create failed");
    file.write_all(name.as_bytes()).expect("write failed");
}

fn read_name() -> String {
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

#[tokio::main]
async fn main() {
    let token = "ODUyMTgxOTQyNjg2MDU2NDg4.YMDGRw.j-Qv5BWiPhKK7C3y4I76-Ks5Bvs";

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
