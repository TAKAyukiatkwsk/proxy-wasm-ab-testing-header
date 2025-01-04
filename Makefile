docker-compose-up:	build
	docker compose up

build:
	cargo build --target wasm32-wasip1 --release
