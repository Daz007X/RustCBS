use corebank::models::{ResponseModel};
use corebank::utility::cbs;


use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/get_sample_core")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/tranform")]
async fn echo(req_body: String) -> impl Responder {

    
    HttpResponse::Ok().body(req_body)
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello) // route /
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/*
fn main()  {
    
    let cbs_data = cbs::get_data();
    println!("CBS DATA : {:#?}",cbs::get_data());
    let cbs_model = ResponseModel::to_model(cbs_data);
    println!("CBS DATA TO JSON MODEL : {:#?}",cbs_model);


   
}*/
