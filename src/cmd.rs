

use log::info;
use tokio::time::Duration;
use crate::{server, client::Client};


pub async fn run(addr: String, port: String) {


    tokio::spawn(async move {

        info!("Start Client ...");
        tokio::time::sleep(Duration::from_secs(1)).await;

        let mut client = Client::new(port).await;
        client.start_test().await;

    });



    // Run Server
    server::run(addr).await
}