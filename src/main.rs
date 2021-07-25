use futures::{SinkExt, StreamExt, channel::mpsc};
use warp::{Filter, fs, ws::*};

mod board;
use board::Board;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "server");
    env_logger::init();

    let file 
        = fs::dir("resource");
        
    let ws 
        = warp::path("ws")
        .and(warp::ws())
        .map(|ws: Ws| {
            ws.on_upgrade(|websocket| async {
                let (ws_tx, ws_rx) = websocket.split();
                let (fc_tx, fc_rx) = mpsc::unbounded();
                let forward = fc_rx.forward(ws_tx);
                let board = Board::new();
                let handler = ws_rx.for_each(|msg| async {
                    let str = String::from(msg.unwrap().to_str().unwrap());
                    if str.contains("ping") {
                        fc_tx.clone().send(Ok(Message::text("pong"))).await.unwrap();
                    }
                    if str.contains("DOMContentLoaded") {
                        let json = serde_json::to_string(&board).unwrap();
                        fc_tx.clone().send(Ok(Message::text(json))).await.unwrap();
                    }
                });
                futures::pin_mut!(forward, handler);
                futures::future::select(forward, handler).await;
                
            })
        });
    warp::serve(ws.or(file).with(warp::log("server"))).run(([127, 0, 0, 1], 3000)).await;
}
