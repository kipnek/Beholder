use actix_web::{get, web, App, HttpServer, Responder, post};

#[get("/ping")]
async fn ping() -> impl Responder {
    "alive".to_string()
}
#[get("/{agent_uuid}/jobs")]
async fn get_jobs(agent_uuid: web::Path<String>) -> impl Responder {

}

#[post("/{agent_uuid}/{job_id}")]
async fn post_jobs(agent_uuid: web::Path<String>, job_id: web::Path<String>) -> impl Responder {

}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(get_jobs)
            .service(post_jobs)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}