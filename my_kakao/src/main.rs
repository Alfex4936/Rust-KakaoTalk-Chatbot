use actix_web::middleware::{Compress, Logger, NormalizePath};
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
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
    carousel.set_header(
        "2022년 공휴일",
        "FOSS",
        "https://www.ajou.ac.kr/_res/ajou/kr/img/intro/img-slogan08.png",
    );

    let db = &conn;

    for holiday in show_holidays(db).await.unwrap() {
        // println!(
        //     "name: {}, date: {}, day_of_week: {}",
        //     holiday.name, holiday.date, holiday.day_of_week
        // );

        let basic_card = BasicCard::new().set_title(holiday.name).set_desc(format!(
            "날짜: {} ({}요일)",
            holiday.date, holiday.day_of_week
        ));

        carousel.add_card(basic_card.build_card());
    }

    result.add_output(carousel.build());

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}

pub async fn show_holidays(conn: &Mongo) -> Result<Vec<Holiday>, ()> {
    let holiday_collection = conn
        .lock()
        .unwrap()
        .database("foss")
        .collection::<Holiday>("holiday");

    let mut holidays = holiday_collection.find(doc! {}, None).await.unwrap();
    let mut result: Vec<Holiday> = Vec::new();
    while let Some(holiday) = holidays.try_next().await.unwrap() {
        result.push(holiday);
    }

    Ok(result)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(init_mongo().await); // MongoDB 초기화

    println!("\n===> Welcome to Rust KakaoChat bot!");

    // 서버 실행
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone()) // <- db는 이런 식으로 서버로 연동
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
            .wrap(Logger::default())
            .service(get_holidays)
    })
    .bind(SERVER)?
    .run()
    .await
}
