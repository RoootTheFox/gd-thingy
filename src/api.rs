use crate::{StreamMeow, LAST_LEVEL};
use rocket::futures::SinkExt;
use rocket::{get, State};
use rocket_ws::WebSocket;

#[get("/api/v1/recent_levels/ws")]
pub(crate) fn recent_levels_ws(state: &State<StreamMeow>, ws: WebSocket) -> rocket_ws::Channel<'_> {
    let mut rx = state.rx.resubscribe();

    ws.channel(move |mut stream| {
        Box::pin(async move {
            // send last level if available
            if let Some(level) = LAST_LEVEL.lock().await.as_ref() {
                if stream.send(level.clone().into()).await.is_err() {
                    return Ok(());
                }
            }

            loop {
                // we assume that data is a JSON containing the level data
                let data = rx.recv().await.unwrap();
                if stream.send(data.into()).await.is_err() {
                    return Ok(());
                }
            }
        })
    })
}
