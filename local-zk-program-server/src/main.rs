use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    get, middleware, post, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};

struct AppState {
    programs: Mutex<HashMap<String, Vec<u8>>>,
}

#[post("/{name}")]
async fn receive_bytes(
    _req: HttpRequest,
    name: web::Path<String>,
    data: web::Data<AppState>,
    payload: web::Payload,
) -> impl Responder {
    let bytes = payload.to_bytes().await.unwrap();
    let mut programs = data.programs.lock().unwrap();
    programs.insert(name.to_string(), bytes.to_vec());
    println!("Received {} bytes for {}", bytes.len(), name);

    HttpResponse::Ok()
}

#[get("/{name}")]
async fn send_bytes(
    _req: HttpRequest,
    name: web::Path<String>,
    data: web::Data<AppState>,
) -> impl Responder {
    let programs = data.programs.lock().unwrap();
    let bytes = programs.get(name.as_str()).unwrap();
    println!("Sent {} bytes for {}", bytes.len(), name);

    HttpResponse::Ok().body(bytes.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address = "0.0.0.0:8080";
    let shared_state = web::Data::new(AppState {
        programs: Mutex::new(HashMap::new()),
    });
    let server = HttpServer::new(move || {
        App::new()
            .app_data(shared_state.clone())
            .wrap(middleware::DefaultHeaders::new().add(("X-Version", "0.2")))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default().log_target("http_log"))
            .service(receive_bytes)
            .service(send_bytes)
    })
    .workers(1)
    .bind(bind_address)?
    .run();

    println!("Server is running on {}", bind_address);

    server.await
}
