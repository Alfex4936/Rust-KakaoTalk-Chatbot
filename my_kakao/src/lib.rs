// src/lib.rs
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate mongodb;

// 아래 URL에는 mongo+srv//id:password~~~~
// 형태로 된 주소 복사하거나 환경 변수에 넣어서 보호
pub const MONGO_URL: &str = "mongodb+srv://root:k15kudk15kud@seokzero.udoff.mongodb.net/myFirstDatabase?retryWrites=true&w=majority";
pub const SERVER: &str = "0.0.0.0:8010";

// DB Holiday 모델
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Holiday {
    pub name: String,
    pub date: String,
    pub day_of_week: String,
}
