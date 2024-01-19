mod api;
mod srv;

use dash_rs::response::parse_get_gj_levels_response;
use lazy_static::lazy_static;
use rocket::{routes, Config};
use std::collections::HashMap;
use std::net::Ipv4Addr;

struct StreamMeow {
    rx: tokio::sync::broadcast::Receiver<String>,
}

// store last level in lazy_static, String because ListedLevel needs a lifetime
lazy_static! {
    static ref LAST_LEVEL: tokio::sync::Mutex<Option<String>> = tokio::sync::Mutex::new(None);
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let config = Config {
        port: 3457,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
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

            match parse_get_gj_levels_response(&string) {
                Ok(v) => {
                    let level = v.first().unwrap();
                    let data = serde_json::to_string(level).unwrap();

                    tx.send(data.clone()).unwrap();
                    let mut last_level = LAST_LEVEL.lock().await;
                    last_level.replace(data);
                }

                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            }
        }
    });

    let _rocket = rocket::build()
        .manage(StreamMeow { rx })
        .mount("/", routes![srv::version, api::recent_levels_ws])
        .mount("/", rocket::fs::FileServer::from("public"))
        .configure(config)
        .launch()
        .await?;

    Ok(())
}
