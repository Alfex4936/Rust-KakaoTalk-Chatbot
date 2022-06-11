FROM rust:1.63.0

RUN apt-get update -y
RUN rustup update nightly;
RUN rustup default nightly;

WORKDIR /app
COPY . .

RUN cargo build --release

EXPOSE 8010

CMD ["cargo", "run", "--release"]