<div align="center">
    <p style="font-size:60px">카카오톡 챗봇</p>
    <p style="font-size:18px">2022-1 오픈소스SW 수업 기말과제</p>
</div>

# 시작하기

Rust언어로 24시간 운영되는 간단한 카카오톡 챗봇 서버를 만들어보는 프로젝트입니다.

들어가기에 앞서, 이 프로젝트에선 Rust언어, AWS 셋업 등은 자세히 설명을 하지 않습니다.

(이 프로젝트는 목표는 흥미 유발 용도)

다만, 알면 좋을만한 언어 문법, 설정은 같이 포함하고 있습니다.

따라서 언어를 잘 몰라도 일단 따라하면서 익힐 수 있도록 목표 삼았습니다.

# 무엇을 만들 것인가?

우선 이 프로젝트를 보고 만들 수 있는 결과물을 보여드리겠습니다.

<div align="center">
<p>
    <img width="680" src="https://blog.kakaocdn.net/dn/cvfXno/btrcZtC3Lxo/3Qz5ztR3B4P9LnO39SpHL0/img.gif">
</p>

위 프로젝트 소스는 [여기서](https://github.com/Alfex4936/Rust-Server/tree/main/chatbot) 확인 할 수 있습니다.

또는 직접 친구 추가해서 사용해 볼 수 있습니다. [친구 추가하기](https://pf.kakao.com/_RUcxnK)
</div>

---

이 프로젝트에서는 MongoDB에 간단한 데이터(2022년 공휴일)를 저장하고

카카오톡에서 "달력"과 같은 메시지를 챗봇에게 보내면 DB에 저장된 공휴일들을 불러올 예정입니다.

# Why Rust?

왜 Rust, 러스트 언어를 선택하였나?

![mostloved](https://user-images.githubusercontent.com/2356749/169690372-7763324e-864b-4422-be7c-4be85d6b5381.png)

2021년 기준 stackoverflow 개발자들 커뮤니티에서

꾸준히 계속 사랑받는 언어로 뽑히고 있습니다.

워낙 빠른 속도와 안정성 때문에 서버 쪽에서 인기를 받고 있으며

수업 때 보신 C가 최고라며 하는 깐깐한 [Linus Torvalds](https://ko.wikipedia.org/wiki/%EB%A6%AC%EB%88%84%EC%8A%A4_%ED%86%A0%EB%A5%B4%EB%B0%9C%EC%8A%A4)가 리눅스 커널 개발 공식 언어로 추가했습니다.

> “Who knows. That’s not the point. The point is for a project to stay interesting — and to stay fun — you have to play with it.” - Linus Torvalds

## 언어 특징

Rust는 C언어와 비슷한 성능을 낼 수 있는데 메모리 안전(no leak, safe rust 기준)이 가장 큰 특징입니다.

(변수 생명 주기도 컨트롤 가능)

메모리 안전, 데이터 레이스 등을 방지할 수 있는 가장 큰 이유는 [소유권](https://choiseokwon.tistory.com/315)이란 개념입니다.

간단한 예를 들면 변수들은 하나의 소유자만 있고, 소유권을 넘기는 컨트롤을 할 수 있습니다.

따라서 아래와 같은 코드를 사용할 수 없습니다. (y가 x 데이터를 소유함)

```rust
let x = 1;
let y = x;

println!("{}", x); // ERROR: value borrowed here after move
```

그러면 x도 출력하고 y도 출력하고 싶으면 `borrowing`을 해야합니다. (빌려주기)

예를 들면 자신이 `자동차` 한 대를 갖고 있다고 해봅시다.

그러면 사진을 찍어서 친구들에게 줄 수 있고 친구들이 내 차가 어떻게 생긴지 알지만

본인 말고는 자동차를 튜닝할 수 없습니다. 이것이 read-only borrowing입니다. `&borrow`

또한 다른 사람이 튜닝하게 할 수 있지만 오직 동시에 한 사람만 차를 소유할 수 있습니다. (mutable borrow: `&mut borrow`)

```rust
// 예제 코드
struct Book {}  // C의 구조체

fn borrow(b: &Book) {}  // 책 빌리는 함수
fn give(b: Book) {}  // 책 주는 함수

fn main() {
    let book = Book{};  // 책 생성
    
    borrow(&book);  // 책 빌리기
    give(book);  // 책 소유권 전달
    borrow(&book);  // 또 빌리기, ERROR, DEAD
    // value borrowed here after move
    // give하고 borrow를 하기전에 give란 함수에서 다시 소유권을 보내야함
}
```

```rust
// 변수 lifetime
impl<'de> Deserialize<'de> for Button {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
    ...
```

# 코딩 시작

긴 서론을 넘어 코딩을 시작해보십니다.

우선 카카오챗봇 서버를 만들기 위해서는 REST API를 이용해야합니다.

무조건 POST로 된 endpoint에 HTTP 응답은 항상 200 (OK)를 보내야합니다. (오류 상황에서도)

웹 프레임워크는 Rust에서 유명하고 빠른 actix를 사용할 것입니다.

## 셋업
<details><summary><b>Rust언어 설치하기</b></summary>

1. Rust 공식 홈페이지에서 [@다운로드](https://www.rust-lang.org/tools/install)

2. 새로운 프로젝트 생성 (cmd/powershell/bash):

    ```sh
    $ cargo new my_kakao
    ```
    
    ![image](https://user-images.githubusercontent.com/2356749/169691825-00e754ed-0331-4b0b-9d11-ab09e4110d77.png)

3. 현재 디렉토리에 my_kakao란 폴더로 이동:

    ```sh
    $ cd my_kakao
    ```

4. `Cargo.toml` 수정:

    dependencies 아래에 코드처럼 바꿔주세요.
    외부 라이브러리 사용할 때 cargo.toml에 적으면 됩니다.

    ```toml
    [dependencies]
    actix-rt = "2"
    actix-http = "3"
    actix-web = "4" 
    futures = "0.3"
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    serde_derive = "1.0"
    mongodb = "2"
    kakao-rs = "0.3"

    [profile.dev]
    opt-level = 0
    debug = true

    [profile.release]
    opt-level = 3
    ```

</details>

<details><summary><b>AWS EC2 셋업</b></summary>

1. `365일`동안 계정을 새로 만들었다면 무료로 24시간 운영할 수 있다.

    EC2는 아마존 서버에 원하는 운영체제를 설치해 빌릴 수 있는 공간이다.

    수업 때 하는 oss@git.ajou.ac.kr 서버랑 똑같다.

    근데 이 셋업 과정이 좀 길고 귀찮다.

    이 [@서버](https://ap-northeast-2.console.aws.amazon.com/console/home?region=ap-northeast-2#)로 들어가서 계정을 만든다.

    그러면 메인 화면에 `EC2`가 있는데 클릭해서 `인스턴스 시작` 버튼을 눌러

    원하는 OS를 설치한다. (Ubuntu 20.04 LTS 설치하세요)

    ![image](https://user-images.githubusercontent.com/2356749/169937872-5db72fcc-032f-4f5d-93a3-d769c2f53e16.png)

2. pem 키, 인증 파일을 받아서 `ssh -i my.pem 주소` 로 접속하는게 보통인데

    파일 경로를 항상 입력해야해서 oss 처럼 비밀번호로 접속할 것이다.

    내 인스턴스를 클릭하고 우측 `연결` 버튼을 눌러서 웹에서 접속한다.

    ![image](https://user-images.githubusercontent.com/2356749/169938301-b337c7ae-ebfc-4767-9778-9e31364319e9.png)

    먼저 계정 비밀번호를 설정한다.

    ```sh
    $ sudo passwd ubuntu
    ```

    원하는 비밀번호를 입력한 후

    ```sh
    $ sudo vim /etc/ssh/sshd_config
    ```

    위 명령어를 입력하고 `PasswordAuthentication yes`를 해준다.

    ![image](https://user-images.githubusercontent.com/2356749/169939359-a953812a-3bcf-415c-a332-1707aa821b4e.png)

    ```sh
    $ sudo service sshd restart
    ```

    그러면 본인 컴퓨터 cmd/powershell/terminal에서 아래를 입력하면

    접속이 성공한다. 주소는 ec2- 로 시작하는 것을 찾는다. (인스턴스 정보에서)

    ```sh
    $ ssh ubuntu@인스턴스_퍼블릭_IPv4_DNS_주소_입력하세요
    ```

    익숙한 그 화면을 볼 수 있다.
    
    ![image](https://user-images.githubusercontent.com/2356749/169940234-7edbcddf-5176-490e-be64-f465d9abe77a.png)

TIP

    서버 주소를 고정하고 싶으면 elastic ip 하나를 부여하면 된다. (한 인스턴스 연결 무료)

    주의할 점은 인스턴스를 1년 후에 종료하고 elastic ip 해제를 안하면 이것도 돈이 나간다.

</details>

<details><summary><b>MongoDB 셋업</b></summary>

1. 공휴일 데이터 입력하기

    수동으로 입력하던지 아래 json 파일을 만들어서 `Add Data - Import File` 한다.

    ```json
    [
    {
        "name": "제 8회 전국동시지방선거",
        "date": "6월 1일",
        "day_of_week": "수"
    },
    {
        "name": "현충일",
        "date": "6월 6일",
        "day_of_week": "월"
    },
    {
        "name": "광복절",
        "date": "8월 15일",
        "day_of_week": "월"
    },
    {
        "name": "추석",
        "date": "9월 9일 ~ 12일",
        "day_of_week": "금 ~ 월"
    },
    {
        "name": "개천절",
        "date": "10월 3일",
        "day_of_week": "월"
    },
    {
        "name": "한글날 대체공휴일",
        "date": "10월 10일",
        "day_of_week": "월"
    },
    {
        "name": "크리스마스",
        "date": "12월 25일",
        "day_of_week": "일"
    }
    ]
    ```

    ![image](https://user-images.githubusercontent.com/2356749/169946784-8579db78-4df8-49ce-bbec-dd1e6e591af9.png)

</details>

<details><summary><b>Rust MongoDB 연동</b></summary>

1. `src/lib.rs` 수정

    사용할 library 및 전역 변수들을 `lib.rs`에 주로 넣어줍니다.

    `SERVER`에 원하는 포트와 주소를 적어도 됩니다.
    
    `0.0.0.0`으로 하면 모든 사람이 접속 가능한 서버가 열립니다.

    `MONGO_URL`은 시스템 환경변수 편집에서 `MONGODB_URL`에다 넣거나

    아예 주소로 그냥 바꿔도 됩니다.

    ```rust
    // src/lib.rs
    #![feature(proc_macro_hygiene, decl_macro)]

    #[macro_use]
    extern crate serde_derive;
    #[macro_use]
    extern crate serde_json;

    extern crate mongodb;

    // 아래 URL에는 mongo+srv//id:password~~~~
    // 형태로 된 주소 복사하거나 환경 변수에 넣어서 보호
    pub const MONGO_URL: &str = env!("MONGODB_URL");
    pub const SERVER: &str = "0.0.0.0:8010";

    // DB Holiday 모델
    #[derive(Serialize, Deserialize, Debug, Default, Clone)]
    pub struct Holiday {
        pub name: String,
        pub date: String,
        pub day_of_week: String,
    }

    ```

2. `src/main.rs` 메인 함수 편집

    프로그램을 실행하면 main 함수가 실행됩니다.

    ```rust
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

        for holiday in show_holidays(db).await.unwrap() {
            println!(
                "name: {}, date: {}, day_of_week: {}",
                holiday.name, holiday.date, holiday.day_of_week
            );

            let basic_card = BasicCard::new()
                .set_title(holiday.name)
                .set_desc(format!("{}", holiday.date))
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
    ```

</details>

<details><summary><b>ㅋㅋㅋ</b></summary>

1. Install the preset:

    ```sh
    $ npm install --save-dev size-limit @size-limit/file
    ```

2. Add the `size-limit` section and the `size` script to your `package.json`:

    ```diff
    + "size-limit": [
    +   {
    +     "path": "dist/app-*.js"
    +   }
    + ],
      "scripts": {
        "build": "webpack ./webpack.config.js",
    +   "size": "npm run build && size-limit",
        "test": "jest && eslint ."
      }
    ```

3. Here’s how you can get the size for your current project:

    ```sh
    $ npm run size

      Package size: 30.08 kB with all dependencies, minified and gzipped
    ```

4. Now, let’s set the limit. Add 25% to the current total size and use that as
   the limit in your `package.json`:

    ```diff
      "size-limit": [
        {
    +     "limit": "35 kB",
          "path": "dist/app-*.js"
        }
      ],
    ```

5. Add the `size` script to your test suite:

    ```diff
      "scripts": {
        "build": "webpack ./webpack.config.js",
        "size": "npm run build && size-limit",
    -   "test": "jest && eslint ."
    +   "test": "jest && eslint . && npm run size"
      }
    ```

6. If you don’t have a continuous integration service running, don’t forget
   to add one — start with [Travis CI].

</details>
