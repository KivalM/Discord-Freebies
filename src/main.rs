use async_std::task;
use serenity::{
    async_trait,
    client::Context,
    model::{
        channel::Message, channel::ReactionType, gateway::Activity, gateway::Ready, id::ChannelId,
        user::OnlineStatus,
    },
    prelude::*,
};
use std::io::Read;
use std::io::Write;
use std::process::exit;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio;

static EPIC_LINK: &str = "https://www.epicgames.com/store/en-US/free-games";

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

async fn get_games() -> Vec<String> {
    let mut caps = DesiredCapabilities::firefox();
    let _ = caps.add_firefox_arg("--headless");
    let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps)
        .await
        .unwrap();
    //navigate to url
    driver.get(EPIC_LINK).await.unwrap();

    let doc = driver
        .find_elements(By::XPath("//a[contains(., \"Free Now\")]"))
        .await
        .unwrap();
    let mut result: Vec<String> = Vec::new();

    for i in &doc {
        let lines = i.text().await.unwrap();
        let line: Vec<&str> = lines.split("\n").collect();
        let name = line[1];
        let link = i.get_property("href").await.unwrap().unwrap();
        result.push(name.to_owned());
        result.push(link);
    }
    let _ = driver.quit().await;
    println!("here");
    return result;
}
async fn send_message(ctx: Context, id: u64, mess: String) {
    let channel = ChannelId(id);
    let _ = channel.say(&ctx.http, mess).await;
}

async fn fmt_string(list: Vec<String>) -> String {
    let mut result = "Beep Boop :: Free Game Alert\n".to_owned();
    let mut i = 0;
    println!("{}", list.len());
    while i < list.len() {
        result = result + "\n\n";
        result = result + "**" + &list[i] + "**\n";
        i = i + 1;
        result = result + &list[i];
        i = i + 1;
    }
    return result;
}
async fn check(channels: Vec<u64>, ctx: Context) {
    let prev_name = read_name();
    let games = get_games().await;
    let curr_name = games[2].to_owned();
    println!("{}", prev_name);
    println!("{}", curr_name);
    if curr_name == prev_name {
        println!("{}", "No New games");
    } else {
        let y = fmt_string(games).await;
        println!("s");
        save_name(&curr_name);
        for i in channels {
            send_message(ctx.to_owned(), i, y.to_owned()).await;
        }
    }
}

async fn cahnnels(ctx: Context) {
    let mut x = Vec::new();
    x.push(745229746727551020);
    x.push(841773185128464415);
    check(x, ctx).await;
}

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == ".ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == ".info" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "I run on rust btw").await {
                let _ = msg.react(&ctx.http, ReactionType::from('👍')).await;
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with(".set_status") {
            let new = msg.content.split_once(" ").unwrap().1;
            let _ = msg.react(&ctx.http, ReactionType::from('👍')).await;
            let _ = ctx
                .set_presence(Some(Activity::playing(new)), OnlineStatus::Online)
                .await;
        } else if msg.content == ".update" {
            let _ = msg.react(&ctx.http, ReactionType::from('👍')).await;
            cahnnels(ctx).await;
        } else if msg.content == ".kys" {
            let _ = msg.react(&ctx.http, ReactionType::from('😔')).await;
            let _ = msg.channel_id.say(&ctx.http, "It's getting dark...").await;
            exit(0);
        } else if msg.content.starts_with(".say") {
            println!("here");
            let _ = msg.react(&ctx.http, ReactionType::from('👍')).await;
            let new = msg.content.split_once(" ").unwrap().1;
            let data = new.split_once(" ").unwrap();
            let channel = data.0;
            let message = data.1;
            println!("{} {}", message, channel);
            let id: u64 = channel.parse().unwrap_or(0);
            println!("{}", id);
            send_message(ctx, id, message.to_owned()).await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        loop {
            cahnnels(ctx.to_owned()).await;
            task::sleep(Duration::from_secs(3600)).await;
        }
    }
}

async fn bot() {
    let token = "NzM1MDg1NzcxODE5NzEyNTgy.XxbH-Q.NrcGUuYB39_jeePlLHF--AMW_T0";

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
#[tokio::main]
async fn main() {
    bot().await;
}
