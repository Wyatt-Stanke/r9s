use headless_chrome::{Browser, LaunchOptionsBuilder};
use owo_colors::OwoColorize;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::time::{sleep, Duration};

async fn wait_for_enter_key(prompt: &str) {
    let mut stdin = BufReader::new(tokio::io::stdin());
    let mut line = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    stdin.read_line(&mut line).await.unwrap();
}

// Gets the replit authentification token from the browser (using 'headful' mode so you can sign in, the "connect.sid" cookie is the token)
pub async fn get_token() -> String {
    let launch_options = LaunchOptionsBuilder::default()
        .headless(false)
        .build()
        .unwrap();
    let browser = Browser::new(launch_options).unwrap();
    let tab = browser.new_tab().unwrap();
    tab.navigate_to("https://replit.com/login").unwrap();

    println!("{}", "Please sign in to your replit account".green());
    wait_for_enter_key("Press enter to open a browser").await;

    while !tab
        .get_cookies()
        .unwrap()
        .iter()
        .any(|cookie| cookie.name == "connect.sid")
    {
        sleep(Duration::from_millis(100)).await;
    }

    tab.get_cookies()
        .unwrap()
        .iter()
        .find(|cookie| cookie.name == "connect.sid")
        .unwrap()
        .value
        .clone()
}
