use std::{self, io::{Result}};
use actix_web::{
                self,
                get, 
                // Responder,
                HttpRequest,
                HttpResponse,
                http::{
                    StatusCode,
                    header::{
                        // self,
                        ContentType
                    }
                }
            };

#[get("/")]
pub async fn run(_req: HttpRequest) -> Result<HttpResponse> {
    // println!("{req:?}");

    // response
    Ok(HttpResponse::build(
        StatusCode::OK
    )
        .content_type(ContentType::plaintext())
        .body("Pong"))
}