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
    async fn snapshot_and_pause(cpu_snapshot_path: String, memory_snapshot_path: String, port: u16, resume: bool) -> String;
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

#[derive(Serialize, Debug)]
#[serde(crate = "rocket::serde")]
struct CreateResponse {
    pid: i32,
    port: i32
}



async fn rpc_call(body: Json<SnapshotRequest<'_>>) -> anyhow::Result<String> {
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), body.rpc_port);
    let transport = tarpc::serde_transport::tcp::connect(socket, newJson::default);
    let client = WorldClient::new(client::Config::default(), transport.await?).spawn();
    let hello = async move {
        // Send the request twice, just to be safe! ;)
        // if body.resume{
        //     tokio::select! {
        //         hello1 = client.snapshot_and_resume(context::current(), body.cpu_snapshot_path.to_string(), body.memory_snapshot_path.to_string(), body.rpc_port) => { hello1 }
        //         // hello2 = client.hello(context::current(), format!("{}2", "Ronak")) => { hello2 }
        //     }
        // }
        // else{
            tokio::select! {
                hello1 = client.snapshot_and_pause(context::current(), body.cpu_snapshot_path.to_string(), body.memory_snapshot_path.to_string(), body.rpc_port, body.resume) => { hello1 }
                // hello2 = client.hello(context::current(), format!("{}2", "Ronak")) => { hello2 }
            }
        // }
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

#[post("/snapshot", data = "<body>")]
async fn snapshot(body: Json<SnapshotRequest<'_>>) -> Result<Json<&'_ str>, MyError>  {
    println!("Snapshot working {:?}", body);
       
    // TODO: Call to vmm rpc, pass on flag to the function
    let a = rpc_call(body).await;
    // WorldClient is generated by the service attribute. It has a constructor `new` that takes a
    // config and any Transport as input.
    match a {
        Ok(msg) => {
            let success = String::from("Success");
            match msg{
                success => return Ok(Json("Snapshot taken successfully")),
                _ => return Err(MyError::build(500, Some("Error".to_string()))),
            }
        },
        Err(e) => {
            return Err(MyError::build(500, Some(e.to_string())));
        }
    }
}

#[post("/create", data = "<body>")]
fn create(body: Json<CreateRequest<'_>>) -> Result<Json<CreateResponse>, MyError> {
    println! ("OK responding for create request {:?}", body);
    // call to the function through rpc call 
    let new_ip = "127.0.0.1";
    let mut new_port : String;
    // TODO: This port needs to be decided based on available ports
    match port_check::free_local_port() {
        Some(port) => new_port = port.to_string(),
        None => return Err(MyError::build(500, Some("Error".to_string()))),
    };

    if body.resume == true {
        // note: In this case resume is called
        // note: Take cpu_path and memory_path.
        // note: Execute VMM binary using memory_path as kernel path cmdline arg and assigning some random port?? to it for RPC.
        match fork() {
            Ok(Fork::Child) => {
                let err = exec::Command::new("../732-demo/target/debug/vmm-reference")
                .arg("--cpu_path")
                .arg(body.cpu_snapshot_path)
                .arg("--memory_path")
                .arg(body.memory_snapshot_path)
                .arg("--port")
                .arg(new_port)
//                 .arg("--ip")
//                 .arg(new_ip)
                .exec();
                println!("Error: {}", err);
            },
            Ok(Fork::Parent(pid)) => {
                println!("pid = {:?}", pid);
                return Ok(Json(CreateResponse { pid, port: new_port.parse::<i32>().unwrap() }));
            },
            Err(e) => {
                return Err(MyError::build(500, Some(format!("Forking failed: {}", e))));
            },
        }
        // note: call resume_vm providing the cpu_path and execute vmm_run.
        // note: return the IP, port as response
    } else {
        // note: In this case fresh VM is to be created from the kernel image
        // note: Take kernel_path
        // note: Execute VMM binary using kernel_path by assigning some random port to it for RPC.
        // note: return the IP, port as response
        match fork() {
            Ok(Fork::Child) => {
                let err = exec::Command::new("../732-demo/target/debug/vmm-reference")
                .arg("--kernel")
                .arg(format!("path={}", body.kernel_path))
                .arg("--port")
                .arg(new_port)
//                 .arg("--ip")
//                 .arg(new_ip)
                .exec();
            },
            Ok(Fork::Parent(pid)) => {
                println!("pid = {:?}", pid);
                return Ok(Json(CreateResponse { pid, port: new_port.parse::<i32>().unwrap() }));
            },
            Err(e) => {
                return Err(MyError::build(500, Some(e.to_string())));
            }
        }
    }
    return Err(MyError::build(500, Some("Error".to_string())));    
}


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![snapshot, create])
}
