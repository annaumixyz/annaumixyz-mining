APP=annaumixyz-mining

build:
	cargo build --release

run:
	cargo run --release

clean:
	rm -rf target

install:
	mkdir -p bin
	cp target/release/$(APP) bin/$(APP)
