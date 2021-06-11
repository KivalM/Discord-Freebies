

use serenity::{
    async_trait,
    client::bridge::gateway::{ShardId, ShardManager},
    http::Http,
    model::{
        channel::{Channel, Message},
        gateway::Ready,
        id::UserId,
        permissions::Permissions,
        user::OnlineStatus,
        id::ChannelId,
        gateway::Activity,
    },
    prelude::*,
    gateway::Shard,
    client::Context,
};
use std::io::Read;
use std::io::Write;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio;
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
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let _ = ctx
            .set_presence(
                Some(Activity::playing("Written in rust")),
                OnlineStatus::Online,
            )
            .await;
        loop {
            let mut caps = DesiredCapabilities::firefox();
            // caps.add_firefox_arg("--headless");
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

            let s: String = format!(
                "//a[contains(@href, \"{}\")]",
                text.to_lowercase().replace(" ", "-")
            );
            let s_slice: &str = &s[..];
            let link = elem
                .find_element(By::XPath(s_slice))
                .await
                .unwrap()
                .get_property("href")
                .await
                .unwrap()
                .unwrap();

            driver.quit().await;

            let channel_to = ChannelId(745229746727551020);

            let gamez = read_name();
            println!("{}", gamez);
            if gamez != text {
                save_name(&text);
                let m: String = format!("**New Free Game** \n\n **{}** \n {}", text, link);
                let m_slice: &str = &m[..];
                let y = channel_to.say(&ctx.http, m_slice).await;
            } else {
                println!(":<>")
            }

            
            println!("restarting");
        }
    }
}

#[tokio::main]
async fn main() {
    let token = "NzM1MDg1NzcxODE5NzEyNTgy.XxbH-Q.ZPTosJeqDjejYuFghgAe__9dI-g";

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
