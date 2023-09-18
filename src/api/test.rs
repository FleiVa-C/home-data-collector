use actix_web::{get, HttpRequest};


#[get("/")]
pub async fn index(req: HttpRequest) -> &'static str {
    println!("{:?}", req);
    "Hello World\r\n"
}