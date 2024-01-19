use crate::StreamMeow;
use dash_rs::response::parse_get_gj_levels_response;
use rocket::futures::SinkExt;
use rocket::{get, State};
use rocket_ws::WebSocket;

#[get("/api/v1/recent_levels/ws")]
pub(crate) fn recent_levels_ws(state: &State<StreamMeow>, ws: WebSocket) -> rocket_ws::Channel<'_> {
    let mut rx = state.rx.resubscribe();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                let data = rx.recv().await.unwrap();
                match parse_get_gj_levels_response(&data) {
                    Ok(v) => {
                        let level = v.first().unwrap();
                        let data = serde_json::to_string(level).unwrap();

                        let _ = stream.send(data.into()).await;
                    }

                    Err(e) => {
                        println!("Error: {}", e);
                        continue;
                    }
                }
            }
        })
    })
}
