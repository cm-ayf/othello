use futures::{FutureExt, StreamExt};
use warp::{Filter, fs, ws};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "server");
    env_logger::init();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let file 
        = fs::dir("resource");
        
    let ws 
        = warp::path("ws")
        .and(warp::ws())
        .map(|ws: ws::Ws| {
            ws.on_upgrade(|websocket| {
                let (tx, rx) = websocket.split();
                rx.forward(tx).map(|result| {
                    if let Err(e) = result {
                        eprintln!("websocket error: {:?}", e);
                    }
                })
            })
        });
    warp::serve(ws.or(file).with(warp::log("server"))).run(([127, 0, 0, 1], 3000)).await;
}
