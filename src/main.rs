use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;
use std::{error::Error, net::Ipv4Addr};

#[derive(Debug, Serialize)]
struct Proc {
    name: String,
    pid: usize,
    cpu_util: f32,
    mem_usage: f32,
}

#[get("/procs")]
pub async fn get_procs() -> Result<impl Responder> {
    let procs = [
        Proc {
            name: "imaging".to_string(),
            pid: 123,
            cpu_util: 0.1,
            mem_usage: 23.3,
        },
        Proc {
            name: "controller".to_string(),
            pid: 124,
            cpu_util: 0.13,
            mem_usage: 25.0,
        },
        Proc {
            name: "actuator".to_string(),
            pid: 125,
            cpu_util: 0.2,
            mem_usage: 10.0,
        },
    ];

    Ok(web::Json(procs))
}

#[get("/procs/{name}")]
pub async fn get_procs_name(name: web::Path<String>) -> Result<impl Responder> {
    let proc = Proc {
        name: name.to_string(),
        pid: 125,
        cpu_util: 0.2,
        mem_usage: 10.0,
    };
    Ok(web::Json(proc))
}

#[allow(unused)]
#[derive(Debug, Serialize)]
enum RoverdStatus {
    Booting,
    Running,
    Failed,
}

#[derive(Debug, Serialize)]
struct Status {
    status: RoverdStatus,
    version: String,
}

#[get("/status")]
pub async fn get_status() -> Result<impl Responder> {
    let status = Status {
        status: RoverdStatus::Running,
        version: "1.0.0".to_string(),
    };
    Ok(web::Json(status))
}

#[derive(Debug, Serialize)]
enum RoverdCmd {
    Start,
    Stop,
    Build,
    Download,
    Update,
}



#[get("/cmd")]
pub async fn get_cmd() -> Result<impl Responder> {
    
    Ok(HttpResponse::Ok().body("cmd"))
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
    let port = 8080;

    println!("listening on port: {port}");

    HttpServer::new(move || {
        App::new()
            .service(get_procs)
            .service(get_procs_name)
            .service(get_status)
            .service(get_cmd)
    })
    .bind((Ipv4Addr::UNSPECIFIED, port))?
    .run()
    .await
}
