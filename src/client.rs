use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, WebSocketStream, MaybeTlsStream, tungstenite::Message};

use crate::server::service::{RequestServerInfoField, DataRequest};



pub struct Client {
    socket: WebSocketStream<MaybeTlsStream<TcpStream>>
}

impl Client {

    pub async fn new(addr: String) -> Self {
        
        let uri = url::Url::parse(&format!("ws://localhost:{}", addr)).unwrap();
        let (socket, _) =  
                connect_async(uri).await.unwrap();

            
        Client {
            socket
        }
    }

    pub async fn start_test(&mut self) {
        
        let req_1 = RequestServerInfoField {
            request: crate::server::service::Request::Ping,
            data_request: DataRequest::Empty,
            reply_to: "84a0c091-5ba8-47db-9d2f-9b4aad197366".to_string(),
        };

        let req_2 = RequestServerInfoField {
            request: crate::server::service::Request::GetServerInfo,
            data_request: DataRequest::Field { field: "Uptime".to_string() },
            reply_to: "4b1e4202-94fe-4159-81d5-caa52b379bf1".to_string(),
        };


        let req_3 = r#"{"request": "GetServerInfo", "data_request": {"field": "abc"}, "reply_to": "15ff134f-dcd7-4b1a-aa32-9e0b2da36dcf"}"#;


        println!("");
        println!("");

        let json = serde_json::to_string(&req_1).unwrap();
        self.socket.send(Message::Text(json.to_string())).await.unwrap();
        let res = self.socket.next().await.unwrap().unwrap();
        println!(">>> {}", json);
        println!("<<< {}", res.to_string());
        

        println!("");

        
        let json = serde_json::to_string(&req_2).unwrap();
        self.socket.send(Message::Text(json.to_string())).await.unwrap();
        let res = self.socket.next().await.unwrap().unwrap();
        println!(">>> {}", json);
        println!("<<< {}", res.to_string());


        println!("");


        let json = req_3;
        self.socket.send(Message::Text(json.to_string())).await.unwrap();
        let res = self.socket.next().await.unwrap().unwrap();
        println!(">>> {}", json);
        println!("<<< {}", res.to_string());


        let _ = self.socket.close(None).await;


    }
}

