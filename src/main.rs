mod actors;
mod routes;
mod wasmFunction;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer, middleware};
use actix::Actor;
use actors::WasmEngineActor;
use routes::{execute_fn, test, upload_fn, get_metrics, get_function_metrics};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start WasmEngineActor
    let wasm_actor = WasmEngineActor {}.start();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(wasm_actor.clone()))
            .wrap(Cors::permissive())  // Enables CORS for all requests
            .wrap(middleware::Logger::default()) // Logging Middleware
            .service(test)
            .service(execute_fn)
            .service(upload_fn)
            .service(get_metrics)
            .service(get_function_metrics)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
