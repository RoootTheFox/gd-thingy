use crate::StreamMeow;
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
                let level: crate::gd_parsing::structs::GeometryDashLevel = data.clone().into();
                println!("{:?}", level);
                let _ = stream.send(data.into()).await;
            }
        })
    })
}
