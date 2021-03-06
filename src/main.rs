

mod cmd;
mod server;
mod client;


use std::env;


#[tokio::main]
async fn main() {
    
    env_logger::init();

    let port = env::args().nth(1).unwrap_or_else(|| "8089".to_string());
    let addr = format!("127.0.0.1:{}", port);
    

    cmd::run(addr, port).await

}


