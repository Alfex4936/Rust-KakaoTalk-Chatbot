TARGET_ARCH=armv7-unknown-linux-gnueabihf

.PHONEY:

build:
	cargo build --release

rasp:
	cargo build --release --target=${TARGET_ARCH}
	cp target/armv7-unknown-linux-gnueabihf/release/my_kakao my_kakao
	scp -P 11112 my_kakao pi@14.33.20.99:~/
	rm my_kakao

linux:
	cargo build --release



all: build run