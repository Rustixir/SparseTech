use futures_util::{StreamExt, SinkExt};
use std::{net::SocketAddr, sync::Arc};
use log::{info, error};
use tokio::net::{TcpStream};
use tokio_tungstenite::{tungstenite::{Error, Message}, accept_async};
use super::service::{SparseSvc, RequestServerInfoField, ResponseServerInfoField, DataResponse, self};




pub async fn accept_connection(arc_svc: Arc<SparseSvc>, peer: SocketAddr, stream: TcpStream) {
    
    if let Err(e) = handle_connection(arc_svc, peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

pub async fn handle_connection(arc_svc: Arc<SparseSvc>, peer: SocketAddr, stream: TcpStream) -> Result<(), Error> {
    let mut ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(_) => return Err(Error::ConnectionClosed)
    };

    
    info!("New WebSocket connection: {}", peer);
    

    // Add one to clients
    arc_svc.subscribe();
    

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {

            // -------------------------------
            
            match endpoint(&arc_svc, msg) {
                Ok(ok_resp) => {
                    ws_stream.send(Message::Text(ok_resp)).await?
                }
                Err(err_resp) => {
                    ws_stream.send(Message::Text(err_resp)).await?;
                    break;
                }
            }

            // -------------------------------
        }
    }

    // Subtract one from clients
    arc_svc.unsubscribe();

    info!("Close connection: {}", peer);

    Ok(())
}





#[inline]
fn endpoint(arc_svc: &Arc<SparseSvc>, msg: Message) -> Result<String, String> {
    
    match decode(msg) {
        Ok(req) => {
            let resp = arc_svc.handle_request(req);
            encode(resp)
        }
        Err(resp) => {
            encode(resp)
        }
    } 
}



fn decode(msg: Message) -> Result<RequestServerInfoField, ResponseServerInfoField> {
    match msg.to_text() {
        Ok(str) => {
            let res: Result<RequestServerInfoField, serde_json::Error> = serde_json::from_str(str);
            match res {
                Ok(req) => {
                    Ok(req)                    
                }
                Err(_e) => {
                    Err(ResponseServerInfoField {
                        response: service::Response::ServiceError,
                        data_response: DataResponse::Code { code: "BadRequest".to_string() },
                        reply_to: "".to_string(),
                    })        
                }
            }
        }
        Err(_e) => {
            Err(ResponseServerInfoField {
                response: service::Response::ServiceError,
                data_response: DataResponse::Code { code: "FormatError".to_string() },
                reply_to: "".to_string(),
            })
        }
    }
}


fn encode(resp: ResponseServerInfoField) -> Result<String, String> {
    match serde_json::to_string(&resp) {
        Ok(json_string) => {
            Ok(json_string)
        }
        Err(e) => {
            unsafe { 
                Err(serde_json::to_string(&ResponseServerInfoField {
                    response: service::Response::ServiceError,
                    data_response: DataResponse::Code { code: e.to_string() },
                    reply_to: resp.reply_to,
                }).unwrap_unchecked()) 
            }
        }
    }
}
