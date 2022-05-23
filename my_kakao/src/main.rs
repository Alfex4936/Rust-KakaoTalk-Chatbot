use actix_web::{middleware, post, web, App, HttpResponse, HttpServer, Responder};
use futures::TryStreamExt;
use kakao_rs::prelude::*;
use mongodb::{bson::doc, options::ClientOptions, Client};
use my_kakao::{Holiday, MONGO_URL, SERVER};
use std::sync::Mutex;

type Mongo = Mutex<Client>;

async fn init_mongo() -> Mongo {
    let client_options = ClientOptions::parse(MONGO_URL).await.unwrap();
    Mutex::new(Client::with_options(client_options).unwrap())
}

#[post("/holiday")]
pub async fn get_holidays(conn: web::Data<Mongo>) -> impl Responder {
    let mut result = Template::new();
    let mut carousel = Carousel::new().set_type(BasicCard::id());

    let db = &conn;

    for sched in show_scheds(db).await.unwrap() {
        // println!("id: {}, content: {}", sched.id, sched.content);

        let basic_card = BasicCard::new()
            .set_title(sched.name)
            .set_desc(format!("{}", sched.date))
            .set_thumbnail(
                "https://raw.githubusercontent.com/Alfex4936/kakaoChatbot-Ajou/main/imgs/{}.png",
            );

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

pub async fn show_scheds(conn: &Mongo) -> Result<Vec<Holiday>, ()> {
    let sched_collection = conn
        .lock()
        .unwrap()
        .database("ajou")
        .collection::<Holiday>("schedule");

    let mut scheds = sched_collection.find(doc! {}, None).await.unwrap();
    let mut result: Vec<Holiday> = Vec::new();
    while let Some(sched) = scheds.try_next().await.unwrap() {
        result.push(sched);
    }

    Ok(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(init_mongo().await); // MongoDB 초기화

    // 서버 실행
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // <- db는 이런 식으로 서버로 연동
            .wrap(middleware::Logger::default())
            .service(get_holidays)
    })
    .bind(SERVER)?
    .run()
    .await
}
