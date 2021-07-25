use futures::{SinkExt, StreamExt, channel::mpsc};
use warp::{Filter, fs, ws};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "server");
    env_logger::init();

    let file 
        = fs::dir("resource");
        
    let ws 
        = warp::path("ws")
        .and(warp::ws())
        .map(|ws: ws::Ws| {
            ws.on_upgrade(|websocket| async {
                let (ws_tx, ws_rx) = websocket.split();
                let (fc_tx, fc_rx) = mpsc::unbounded();
                let forward = fc_rx.forward(ws_tx);
                let ping_pong = ws_rx.for_each(|msg| async {
                    if msg.unwrap().to_str().unwrap().contains("ping") {
                        fc_tx.clone().send(Ok(ws::Message::text("pong"))).await.unwrap();
                    }
                });
                futures::pin_mut!(forward, ping_pong);
                futures::future::select(forward, ping_pong).await;
                
            })
        });
    warp::serve(ws.or(file).with(warp::log("server"))).run(([127, 0, 0, 1], 3000)).await;
}
