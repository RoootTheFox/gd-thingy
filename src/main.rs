mod api;
mod srv;

use rocket::config::LogLevel;
use rocket::{routes, Config};
use std::collections::HashMap;
use std::net::Ipv4Addr;

struct StreamMeow {
    rx: tokio::sync::broadcast::Receiver<String>,
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = Config {
        port: 3457,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        log_level: LogLevel::Debug,
        ..Config::default()
    };

    let (tx, rx) = tokio::sync::broadcast::channel(12);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3));

        let client = reqwest::Client::builder()
            .default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert("User-Agent", reqwest::header::HeaderValue::from_static(""));
                headers
            })
            .build()
            .unwrap();
        loop {
            interval.tick().await;

            let mut params = HashMap::new();
            params.insert("secret", "Wmfd2893gb7");
            params.insert("type", "4"); // 4 = recent levels
            params.insert("count", "1"); // we only need the first level

            let response = client
                .post("https://www.boomlings.com/database/getGJLevels21.php")
                .form(&params)
                .send()
                .await
                .unwrap();

            let string = response.text().await.unwrap();

            tx.send(string).unwrap();
        }
    });

    let _rocket = rocket::build()
        .manage(StreamMeow { rx })
        .mount("/", routes![srv::version, api::recent_levels_ws])
        .configure(config)
        .launch()
        .await?;

    Ok(())
}
