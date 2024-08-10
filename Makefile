all: build

build:
	cargo build

release:
	cargo build --release

clean:
	cargo clean

run:
	cargo run

install: release
	sudo cp target/release/goten /usr/local/bin/goten

uninstall:
	sudo rm /usr/local/bin/goten
