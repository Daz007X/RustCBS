use corebank::models::{ResponseModel};
use corebank::utility::cbs;
use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/file")]
async fn read_file() -> impl Responder {
    match cbs::get_data_file().await {
        Ok(contents) => {
            match ResponseModel::to_model(contents) {
                Ok(response_msg) => HttpResponse::Ok().json(response_msg), // ส่ง ResponseModel เป็น JSON
                Err(e) => HttpResponse::BadRequest().body(format!("Error parsing contents: {}", e)),
            }
            },
        Err(e) => HttpResponse::InternalServerError().body(format!("Error reading file: {}", e)),
    }
}

#[post("/tranform")]
async fn tranform_msg(req_body: String) -> impl Responder {
    println!("RSBODY : {}",req_body);
    match ResponseModel::to_model(req_body) 
    {
        Ok(response_msg) => HttpResponse::Ok().json(response_msg), // ส่ง ResponseModel เป็น JSON
        Err(e) => HttpResponse::BadRequest().body(format!("Error parsing contents: {}", e)),
    }
}





#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(read_file)
            .service(tranform_msg)
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
