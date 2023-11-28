use actix_web::{get, web, App, HttpServer, Responder, post};

#[get("/{agent_uuid}/ping")]
async fn ping(agent_uuid: web::Path<String>) -> impl Responder {
    //record last alive result
    "ping"

}
#[get("/{agent_uuid}/jobs")]
async fn get_jobs(agent_uuid: web::Path<String>) -> impl Responder {
    //post get jobs
    "get jobs"
}

#[post("/{agent_uuid}/results")]
async fn post_job_results(agent_uuid: web::Path<String>, job_id: web::Path<String>) -> impl Responder {
    "results"
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .service(get_jobs)
            .service(post_job_results)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}