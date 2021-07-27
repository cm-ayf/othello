use futures::{SinkExt, StreamExt, channel::mpsc, lock::Mutex};
use warp::{Error, Filter, fs, ws::*};

mod board;
use board::Board;

const DEFAULT_PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "server");
    env_logger::init();

    let mut args = std::env::args();
    args.next().unwrap();
    let port = match args.next() {
        Some(str) => {
            match str.parse::<u16>() {
                Ok(u) => u,
                Err(_) => {
                    println!("port argument error; using {}", DEFAULT_PORT);
                    DEFAULT_PORT
                },
            }
        }
        None => DEFAULT_PORT,
    };

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
                let board = Mutex::new(Board::new());
                let handler = ws_rx.for_each(|msg| async {
                    let tx = fc_tx.clone();
                    handler(msg, &board, tx).await;
                });
                futures::pin_mut!(forward, handler);
                futures::future::select(forward, handler).await;

            })
        });
    warp::serve(ws.or(file).with(warp::log("server"))).run(([127, 0, 0, 1], port)).await;
}

async fn handler(msg: Result<Message, Error>, board: &Mutex<Board>, mut tx: mpsc::UnboundedSender<Result<Message, Error>>) {
    let mut board = board.lock().await;
    let str = String::from(msg.unwrap().to_str().unwrap());
    let mut cmd = str.split_whitespace();
    match cmd.next() {
        Some("ping") => tx.send(Ok(Message::text("pong"))).await.unwrap(),
        Some("reload") => {
            let json = serde_json::to_string(&*board).unwrap();
            tx.send(Ok(Message::text(json))).await.unwrap();
        },
        Some("put") => {
            let args = cmd.collect::<Vec<_>>();
            if args.len() < 2 {
                tx.send(Ok(Message::text("put: not enough arguments"))).await.unwrap();
            } else {
                match (args[0].parse::<usize>(), args[1].parse::<usize>()) {
                    (Ok(row_pos), Ok(col_pos)) => {
                        match board.put(row_pos, col_pos) {
                            Ok(b) => {
                                let json = serde_json::to_string(&*board).unwrap();
                                tx.send(Ok(Message::text(json))).await.unwrap();
                                if b {
                                    tx.send(Ok(Message::text("end"))).await.unwrap();
                                }
                            },
                            Err(e) => tx.send(Ok(Message::text(e))).await.unwrap(),
                        }
                    },
                    _ => {
                        tx.send(Ok(Message::text("put: invalid argument: must be usize"))).await.unwrap();
                    }
                }
            }
        }
        Some("close") => {
            return;
        }
        Some(str) => tx.send(Ok(Message::text(format!("invalid command {}", str)))).await.unwrap(),
        None => tx.send(Ok(Message::text("no command parsed"))).await.unwrap(),
    }
}