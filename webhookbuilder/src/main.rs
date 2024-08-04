use serenity::all::ExecuteWebhook;
use serenity::all::Http;
use serenity::all::Webhook;
use serenity::Result;
use std::io::{self, Read};
use tokio;

const content: &str = ""; // Input the Message you want to send
const username: &str = ""; // input the name of the bot
const webhook_data: &str = ""; // input the webhook itself
const send_amount: i32 = 5; // input the amount of times you want

#[tokio::main]
async fn main() -> Result<()> {
    let http = Http::new("");
    let webhook = Webhook::from_url(&http, webhook_data).await?;
    let builder = ExecuteWebhook::new().content(content).username(username);
    let mut amount: i32 = 0;
    println!("Sending webhooks")
    loop {
        webhook.execute(&http, false, builder.clone()).await?;
        amount += 1;
        println!("Webhook Sent")
        if amount >= send_amount {
            break;
        }
    }
    Ok(())
}
