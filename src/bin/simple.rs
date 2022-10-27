#[macro_use] extern crate rocket;

use rocket::http::Status;
use rocket::serde::{Serialize, Deserialize, json::Json};
use fork::{fork, Fork};
use exec::Command;
use std::{error::Error, net::SocketAddr, time::Duration, net::IpAddr, net::Ipv4Addr};
use tarpc::{client, context, tokio_serde::formats::Json as newJson};
use tokio::time::sleep;

#[tarpc::service]
pub trait World {
    /// Returns a greeting for name.
    async fn snapshot_and_resume(cpu_snapshot_path: String, memory_snapshot_path: String, port: u16) -> String;
    async fn snapshot_and_pause(cpu_snapshot_path: String, memory_snapshot_path: String, port: u16) -> String;
}

// use rocket_okapi::gen::OpenApiGenerator;
// use rocket_okapi::okapi;
// use rocket_okapi::okapi::openapi3::{MediaType, Responses};
// use rocket_okapi::response::OpenApiResponderInner;
// use rocket_okapi::OpenApiError;

/// error type
#[derive(Debug, Serialize, schemars::JsonSchema)]
pub struct ErrorContent {
    // HTTP Status Code returned
    code: u16,
    // Reason for an error
    reason: String,
    // Description for an error if any
    description: Option<String>,
}

/// Error messages returned to user
#[derive(Debug, serde::Serialize, schemars::JsonSchema)]
pub struct MyError {
    pub error: ErrorContent,
}

impl MyError {
    // building a custom error.
    pub fn build(code: u16, description: Option<String>) -> MyError {
        let reason: String;
        match code {
            400 => reason = "Bad Request".to_string(),
            401 => reason = "Unauthorized".to_string(),
            _ => reason = "Error".to_string(),
        }
        MyError {
            error: ErrorContent {
                code,
                reason,
                description,
            },
        }
    }
}

// /// Create my custom response
// pub fn bad_request_response(gen: &mut OpenApiGenerator) -> okapi::openapi3::Response {
//     let schema = gen.json_schema::<MyError>();
//     okapi::openapi3::Response {
//         description: "\
//         # 400 Bad Request\n\
//         The request given is wrongly formatted or data was missing. \
//         "
//         .to_owned(),
//         content: okapi::map! {
//             "application/json".to_owned() => MediaType {
//                 schema: Some(schema),
//                 ..Default::default()
//             }
//         },
//         ..Default::default()
//     }
// }

// pub fn unauthorized_response(gen: &mut OpenApiGenerator) -> okapi::openapi3::Response {
//     let schema = gen.json_schema::<MyError>();
//     okapi::openapi3::Response {
//         description: "\
//         # 401 Unauthorized\n\
//         The authentication given was incorrect or insufficient. \
//         "
//         .to_owned(),
//         content: okapi::map! {
//             "application/json".to_owned() => MediaType {
//                 schema: Some(schema),
//                 ..Default::default()
//             }
//         },
//         ..Default::default()
//     }
// }

impl<'r> rocket::response::Responder<'r, 'static> for MyError {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        // Convert object to json
        let body = serde_json::to_string(&self).unwrap();
        rocket::Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::JSON)
            .status(rocket::http::Status::new(self.error.code))
            .ok()
    }
}

// impl OpenApiResponderInner for MyError {
//     fn responses(gen: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
//         use rocket_okapi::okapi::openapi3::RefOr;
//         Ok(Responses {
//             responses: okapi::map! {
//                 "400".to_owned() => RefOr::Object(bad_request_response(gen)),
//                 // Note: 401 is already declared for ApiKey. so this is not essential.
//                 // "401".to_owned() => RefOr::Object(unauthorized_response(gen)),
//             },
//             ..Default::default()
//         })
//     }
// }


#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct SnapshotRequest<'a> {
    cpu_snapshot_path : &'a str,
    memory_snapshot_path : &'a str,
    rpc_port: u16,
    resume: bool,  
}

#[derive(Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct CreateRequest<'a> {
    cpu_snapshot_path : &'a str,
    memory_snapshot_path : &'a str,
    kernel_path: &'a str,
    resume: bool,
}



async fn rpc_call(body: Json<SnapshotRequest<'_>>) -> anyhow::Result<String> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), body.rpc_port);
    let transport = tarpc::serde_transport::tcp::connect(socket, newJson::default);
    let client = WorldClient::new(client::Config::default(), transport.await?).spawn();
    let hello = async move {
        // Send the request twice, just to be safe! ;)
        if body.resume{
            tokio::select! {
                hello1 = client.snapshot_and_resume(context::current(), body.cpu_snapshot_path.to_string(), body.memory_snapshot_path.to_string(), body.rpc_port) => { hello1 }
                // hello2 = client.hello(context::current(), format!("{}2", "Ronak")) => { hello2 }
            }
        }
        else{
            tokio::select! {
                hello1 = client.snapshot_and_pause(context::current(), body.cpu_snapshot_path.to_string(), body.memory_snapshot_path.to_string(), body.rpc_port) => { hello1 }
                // hello2 = client.hello(context::current(), format!("{}2", "Ronak")) => { hello2 }
            }
        }
    }.await;
    match hello {
        Ok(s) => {
            if s == "Success"{
                return Ok(s);
            }else {
                return Ok("Error".to_string());
            }
        }
        Err(_) => {
            return Ok("Error".to_string());
        }
    }
}
// import env
// use env;
pub fn main () {
    let func=std::env::args().nth(1).unwrap();
    // if func is snapshot
    if func == "pause"{
        let cpu_snapshot_path = std::env::args().nth(2).unwrap();
        let memory_snapshot_path = std::env::args().nth(3).unwrap();
        let rpc_port = std::env::args().nth(4).unwrap().parse::<u16>().unwrap();
        let resume = std::env::args().nth(5).unwrap().parse::<bool>().unwrap();
        let body = Json(SnapshotRequest{
            cpu_snapshot_path: &cpu_snapshot_path,
            memory_snapshot_path: &memory_snapshot_path,
            rpc_port: rpc_port,
            resume: resume,
        });
        let result = tokio::runtime::Runtime::new().unwrap().block_on(rpc_call(body));
        match result {
            Ok(s) => {
                if s == "Success"{
                    println!("Success");
                }else {
                    println!("Error");
                }
            }
            Err(_) => {
                println!("Error");
            }
        }
    }

}