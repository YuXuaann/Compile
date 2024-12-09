TASK_ID = 2

ss: build
	cp target/release/task4_${TASK_ID} ss

build:
	cargo build --release

clean:
	rm -f ss
	cargo clean

.PHONY: build clean