TASK_ID = 3

ss: build
	cp target/release/task4_${TASK_ID} ss
	@echo -e "\033[0;32mBuild complete, run ./ss PATH to execute the program\033[0m"

build:
	cargo build --release

clean:
	rm -f ss
	cargo clean

.PHONY: build clean