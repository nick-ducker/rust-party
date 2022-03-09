use actix_web::{ get, post, web, App, HttpResponse, HttpServer, Responder, Result };
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
struct Party {
    bluemountains: bool,
    van: bool,
}

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[post("/update_party")]
async fn update_party(party: web::Json<Party>) -> impl Responder {
    // try and update a local data json
    let mut j = serde_json::to_string(&party).expect("failed to serialize json");
    let mut file = File::open("./party.json").expect("failed to open json file");
    file.read_to_string(&mut j).expect("failed to write to json file");
    
    println!("{:?}", party);
    HttpResponse::Ok()    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(update_party)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
