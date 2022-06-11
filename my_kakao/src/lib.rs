// src/lib.rs
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate mongodb;

// 아래 URL에는 mongo+srv//id:password~~~~
// .env.example 파일에 설정하세요.
pub const MONGO_URL: &str = env!("MONGODB_URL");
pub const SERVER: &str = "0.0.0.0:8010";

// DB Holiday 모델
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Holiday {
    pub name: String,
    pub date: String,
    pub day_of_week: String,
}
