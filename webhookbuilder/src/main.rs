use serenity::all::ExecuteWebhook;
use serenity::all::Http;
use serenity::all::Webhook;
use serenity::Result;
use std::io::{self, Read};
use tokio;

const content: &str = "";
const username: &str = "";
const webhook_data: &str = "";
const send_amount: i32 = 5;

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
