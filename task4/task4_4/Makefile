TASK_ID = 4

ss: build
	@if [ ! -d "result_pic" ]; then mkdir result_pic; fi
	cp target/release/task4_${TASK_ID} ss
	@echo -e "\033[0;32mBuild complete, run ./ss PATH to execute the program\033[0m"

build:
	cargo build --release

clean:
	rm -f ss
	rm -rf result_pic
	cargo clean

.PHONY: build clean