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

1. MongoDB Atlas를 이용해서 무료 클러스터를 만든다. ([@링크](https://www.mongodb.com/cloud/atlas/register))

    ![image](https://user-images.githubusercontent.com/2356749/169950092-a5fa9478-db64-4172-86e1-63494a72a4e2.png)

    `Connect` 버튼을 누르고 `Connect your application`을 누르면 연결 주소가 나온다.

    `mongodb+srv://root~~~~`

2. 공휴일 데이터 입력하기

    MongoDB Compass 프로그램을 이용하면 쉽게

    DB를 확인할 수 있다. (mysql workbench처럼)

    데이터베이스 이름은 `foss`, 이 데이터베이스 안에 `holiday` 란 collection을 만들어서 연결할 것이다.

    수동으로 입력하던지 아래 json 파일을 만들어서 `ADD DATA - Import File` 한다.

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

<details><summary><b>카카오톡 챗봇 API 셋업</b></summary>

1. [i.kakao.com](https://i.kakao.com/)에 들어가 카카오 계정으로 봇을 하나 만든다. (허락 요청은 몇일이 걸릴 수도 있음)

![image](https://user-images.githubusercontent.com/2356749/169967058-85acda5c-ab09-42a1-906b-6ee038230a6a.png)

2. 당장 알아야 할 사항은 `시나리오`, `스킬` 부분이다.

    시나리오는 어떠한 메시지에 어떻게 반응할지 정하는 부분이다.

    스킬에는 서버 endpoint 들을 추가하여 `밥`을 카톡에서 사용자가 보냈을 때 `http://server/bap`

    이런 식으로 연결한다.

    그러면 `+ 시나리오`를 눌러서 아래처럼 만든다. (공휴일, 휴일이란 메시지에 반응할 것이다.)

    ![image](https://user-images.githubusercontent.com/2356749/169967598-aa52d14c-412b-49f6-99ed-cf84727d7fb9.png)

    `스킬`로 이동해서 `추가` 버튼을 누른다.

    ![image](https://user-images.githubusercontent.com/2356749/169968388-1eee5bd4-e20a-418e-9da2-2c94318321c9.png)

    `URL` 부분에는 실제 서비스에 작동할 동작이고 `Test URL`은 웹에서 테스트할 때 작동할 동작을 지정할 수 있는데

    여기선 나눌 필요가 없어서 둘 다 똑같이 설정한다.

    주소는 ssh 연결한 서버에 `:8010` 포트 `/holiday` 엔드포인트 순서이다.

    그러면 카카오톡에서 메시지가 오면 서버에 `POST 서버:8010/holiday`로 요청이 간다. (무조건 POST임)

    그러면 아까 시나리오 부분으로 다시 가서 스킬을 연결해준다.

    ![image](https://user-images.githubusercontent.com/2356749/169968912-aab32d66-7d0f-44f5-a8ee-be8fe706ae4c.png)

3. 메시지 유형

    공식 [@API](https://chatbot.kakao.com/docs/tutorial-chatbot-response)를 참고하면 된다.

    위 메시지들을 전부 JSON 형식으로 만들어서 endpoint 함수에서 return 해줘야한다.

    `fn return_holiday(...) -> JSON`

</details>

# kakao-rs

Rust에서 Rust 데이터들로 JSON을 만들어서 응답을 보내는 것은 까다로울 수 있다.

그래서 라이브러리를 직접 제작해두었다. [@Github 주소 | MIT 라이센스](https://github.com/Alfex4936/kakao-rs)

만들어둔 [@공식 문서](https://docs.rs/kakao-rs/latest/kakao_rs/)를 참고하거나

`README`, `test 폴더`를 확인하면 어떻게 응답을 만드는지 볼 수 있다.

예를 들면 아래와 같은 [ListCard](https://i.kakao.com/docs/skill-response-format#listcard)를 만들기 위해서

![image](https://user-images.githubusercontent.com/2356749/169971426-0a689be3-e6cf-4608-8b31-46402a0a8176.png)

아래와 같은 JSON을 만들어야 하는데

```json
{
  "response": {
    "template": {
      "outputs": [
        {
          "simpleText": {
            "text": "오늘 공지 총 9개"
          }
        },
        {
          "listCard": {
            "buttons": [
              {
                "label": "4개 더보기",
                "action": "message",
                "messageText": "더보기"
              },
              {
                "label": "공유하기",
                "action": "share"
              }
            ],
            "header": {
              "title": "22.05.24) 오늘 공지"
            },
            "items": [
              {
                "title": "[홍보] 2022 수원역 로데오 콘테스트",
                "description": "[기타] LINC사업팀 05.24",
                "link": {
                  "web": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=198483&article.offset=0&articleLimit=30"
                }
              },
              {
                "title": "[메이커스페이스] 운영일자 변경안내 (2022.05.30-06.17)",
                "description": "[기타] 창업지원팀 05.24",
                "link": {
                  "web": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=198479&article.offset=0&articleLimit=30"
                }
              },
              {
                "title": "소원나무 이벤트 5월 3주차 당첨자 발표!",
                "description": "[취업] 대학일자리플러스센터 05.24",
                "link": {
                  "web": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=198473&article.offset=0&articleLimit=30"
                }
              },
              {
                "title": "[중앙도서관] 소음 허용 기준에 따른 도서관 공간 이용 안내",
                "description": "[기타] 중앙도서관 학술정보팀 05.24",
                "link": {
                  "web": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=198471&article.offset=0&articleLimit=30"
                }
              },
              {
                "title": "2022학년도 1학기 학생상담 안내(2022.6.14.까지)",
                "description": "[기타] 공학교육혁신센터 05.24",
                "link": {
                  "web": "https://www.ajou.ac.kr/kr/ajou/notice.do?mode=view&articleNo=198468&article.offset=0&articleLimit=30"
                }
              }
            ]
          }
        }
      ],
      "quickReplies": [
        {
          "action": "message",
          "label": "오늘",
          "messageText": "오늘 공지 보여줘"
        },
        {
          "action": "message",
          "label": "어제",
          "messageText": "어제 공지 보여줘"
        }
      ]
    },
    "version": "2.0"
  }
}
```

만들어둔 라이브러리를 이용하면 다음와 같다.

```rust
#[post("/notice/today")]
pub async fn get_today_notice(kakao: web::Json<Value>) -> impl Responder {
    // println!("{:#?}", kakao);
    let mut result = Template::new();
    result.add_qr(QuickReply::new("오늘", "오늘 공지 보여줘"));
    result.add_qr(QuickReply::new("어제", "어제 공지 보여줘"));

    let mut notices: Vec<Notice> = match notice_parse("ajou", Some(30)).await {
        Ok(yes) => yes,
        _ => {
            result.add_output(SimpleText::new("홈페이지 반응이 늦습니다. :(").build());

            return HttpResponse::Ok()
                .content_type("application/json")
                .body(serde_json::to_string(&result).unwrap());
        }
    };
    let today = Local::now().format("%y.%m.%d").to_string(); // "21.07.20"

    let mut list_card = ListCard::new(format!("{}) 오늘 공지", today));

    // notices.iter().position(|&n| n.date.ne(&today)).unwrap();

    notices = notices
        .into_iter()
        .filter(|notice| notice.date.eq(&today))
        .collect();

    if !notices.is_empty() {
        result.add_output(SimpleText::new(format!("오늘 공지 총 {}개", notices.len())).build());
    }

    if notices.len() > 5 {
        let label = format!("{}개 더보기", notices.len() - 5);
        list_card.add_button(
            Button::new(ButtonType::Text)
                .set_label(label)
                .set_msg("더보기"),
        );
        notices.resize(5, Notice::default());
    } else {
        list_card.add_button(
            Button::new(ButtonType::Link)
                .set_label("아주대 공지")
                .set_link(AJOU_LINK),
        );
    }

    if notices.is_empty() {
        list_card.add_item(ListItem::new("공지가 없습니다!").set_image(
            "http://k.kakaocdn.net/dn/APR96/btqqH7zLanY/kD5mIPX7TdD2NAxgP29cC0/1x1.jpg",
        ));
    } else {
        list_card.add_button(Button::new(ButtonType::Share).set_label("공유하기"));
        for notice in notices.iter_mut() {
            let description = format!(
                "[{}] {} {}",
                notice.category,
                notice.writer,
                notice.date[notice.date.len() - 5..].to_string()
            );

            list_card.add_item(
                ListItem::new((*notice.title).to_string())
                    .set_desc(description)
                    .set_link((*notice.link).to_string()),
            );
        }
    }

    result.add_output(list_card.build()); // moved list_card's ownership

    HttpResponse::Ok()
        .content_type("application/json")
        .body(serde_json::to_string(&result).unwrap())
}
```

지원하는 메시지 유형: [@소스](https://github.com/Alfex4936/kakao-rs/tree/master/src/components)

* QuickReply, Carousel(BasicCard|CommerceCard), ListCard
* SimpleImage, SimpleText
* Button(Call, Share, Link, Text)
* BasicCard, CommerceCard, ItemCard