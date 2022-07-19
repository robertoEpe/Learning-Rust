
use actix_web::*;

async fn saludar() -> impl Responder {
    HttpResponse::Ok().body("<h1 style='color:red'>hola</h1>")
}

#[get("/tweets")]
async fn get_tweets() -> HttpResponse {

    let tweets = ["tweet 1: hola", "teet 2: chao"];

    HttpResponse::Ok()
    .content_type("application/json")
    .json(tweets)
}

#[get("/tweets/{id}")]
async fn get_tweet(path: web::Path<String,>) -> HttpResponse {

    let tweet= format!("Este es el tweet {}", path.0); 

    HttpResponse::Ok()
    .content_type("application/json")
    .json(tweet)
}

#[get("/tweets/{id}/likes")]
async fn get_likes_by_tweet(path: web::Path<String,>) -> HttpResponse {

    let tweet= format!("Este es el tweet {} y sus likes", path.0); 

    HttpResponse::Ok()
    .content_type("application/json")
    .json(tweet)
}

#[post("/tweets/{id}/likes")]
async fn like_tweet(path: web::Path<String,>) -> HttpResponse {

    let tweet= format!("You like the tweet {} and you suck!", path.0); 

    HttpResponse::Created()
    .content_type("application/json")
    .json(tweet)
}

#[delete("/tweets/{id}/likes")]
async fn remove_like(_path: web::Path<String,>) -> HttpResponse {

    HttpResponse::NoContent()
    .content_type("application/json")
    .await.unwrap()
}

#[post("/tweets")]
async fn create_tweet() -> HttpResponse {

    let tweets = ["tweet 1: new", "teet 2: tweet"];

    HttpResponse::Created()
    .content_type("application/json")
    .json(tweets)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    
    HttpServer::new ( || {
        App::new()
        .route("/", web::get().to(saludar))
        .service(get_tweets)
        .service(get_tweet)
        .service(like_tweet)
        .service(remove_like)
        .service(get_likes_by_tweet)
        .service(create_tweet)
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}
