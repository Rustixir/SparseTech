
use tokio::net::{TcpListener};
use std::sync::Arc;
use log::{info};


pub mod service;
pub mod transport;



pub async fn run(addr: String) {

    // Listening on addr
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);
    

    // Create Business svc
    let arc_svc = Arc::new(service::SparseSvc::new());
    
    
    while let Ok((stream, _)) = listener.accept().await {
        
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);

        tokio::spawn(transport::accept_connection(arc_svc.clone(), peer, stream));
    }
}



