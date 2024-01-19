use crate::StreamMeow;
use rocket::futures::SinkExt;
use rocket::{get, State};
use rocket_ws::WebSocket;

#[get("/v")]
pub(crate) fn version() -> String {
    format!("{} {}", env!("CARGO_CRATE_NAME"), env!("CARGO_PKG_VERSION"))
}

#[get("/listen")]
pub(crate) fn listen<'a>(state: &State<StreamMeow>, ws: WebSocket) -> rocket_ws::Channel<'a> {
    let mut rx = state.rx.resubscribe();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            loop {
                let data = rx.recv().await.unwrap();
                let _ = stream.send(data.into()).await;
            }
        })
    })
}
