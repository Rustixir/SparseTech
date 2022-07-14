use std::sync::atomic::{AtomicU64, Ordering};

use serde_derive::{Serialize, Deserialize};




pub trait SparseSerivce {
    
    fn subscribe(&self);
    fn unsubscribe(&self);
    fn handle_request(&self, req: RequestServerInfoField) -> ResponseServerInfoField;

}


pub struct SparseSvc {
    clients: AtomicU64
}

impl SparseSvc {

    pub fn new() -> Self {
        SparseSvc { 
            clients: AtomicU64::new(0)
        }
    }

    pub fn subscribe(&self) {
        self.clients.fetch_add(1, Ordering::AcqRel);
    }

    pub fn unsubscribe(&self) {
        self.clients.fetch_sub(1, Ordering::AcqRel);
    }

    pub fn handle_request(&self, req: RequestServerInfoField) -> ResponseServerInfoField {
        match req.request {
            Request::Ping => {
               ResponseServerInfoField {
                response: Response::Ping,
                data_response: DataResponse::Empty,
                reply_to: req.reply_to,
                }
            }
            Request::GetServerInfo => {
                if let DataRequest::Field { field } = req.data_request {
                    match field.as_str() {
                        "Uptime" | "ConnectedClients" => {
                            ResponseServerInfoField {
                                response: Response::GetServerInfo,
                                data_response: DataResponse::Value { value: self.clients.load(Ordering::Acquire) },
                                reply_to: req.reply_to
                            }
                        }
                        _ => {
                            ResponseServerInfoField {
                                response: Response::GetServerInfo,
                                data_response: DataResponse::Code { code: "InvalidFieldName".to_string() },
                                reply_to: req.reply_to
                            }
                        }
                    }
                } else {
                    // Never Happen
                    todo!()
                }
            }
        }
    }
}





#[derive(Deserialize, Serialize)]
pub struct RequestServerInfoField {
    pub request: Request,
    pub data_request: DataRequest,
    pub reply_to: String
}


#[derive(Deserialize, Serialize)]
pub struct ResponseServerInfoField {
    pub response: Response,
    pub data_response: DataResponse,
    pub reply_to: String
}



// ----------------------------------------

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
pub enum DataRequest {
    Field{ field: String },
    Empty
}


#[derive(Deserialize, Serialize)]
#[serde(untagged)] 
pub enum DataResponse {
    Value{ value: u64 },
    Code{ code: String },
    Empty
}



#[derive(Deserialize, Serialize)]
pub enum Request {
    Ping,
    GetServerInfo
}

#[derive(Deserialize, Serialize)]
pub enum Response {
    Ping,
    GetServerInfo,
    ServiceError,
}

