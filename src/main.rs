use warp::{Filter, fs};

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "server");
    env_logger::init();

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let file 
        = fs::dir("resource")
        .with(warp::log("server"));
    warp::serve(file).run(([127, 0, 0, 1], 3000)).await;
}
