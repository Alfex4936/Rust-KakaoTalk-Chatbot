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
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"
    serde_derive = "1.0"
    mongodb = "2"
    kakao-rs = "0.3"

    [profile.dev]
    opt-level = 0

    [profile.release]
    opt-level = 3
    ```

5. `struct Holiday`:

    각 공휴일마다 갖는 데이터를 적어줍니다.
    (DB 모델 따라서)

    ```rust
    pub struct Holiday {
        pub name: String, // varchar, 명칭
        pub date: String, // varchar, 날짜
        pub day_of_week: String, // vachar, 요일
    }
    ```
</details>

<details><summary><b></b></summary>

1. Rust와 MongoDB 연결하기

    

</details>

<details><summary><b>사용법 보기</b></summary>

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