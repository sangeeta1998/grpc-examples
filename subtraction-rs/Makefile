all: build

build:
	# Create statically-linked binary
	mkdir -p /tmp/out
	cargo build --release

	# Stage statically-linked binaries
	mkdir -p out
	cp target/release/subtractor-rs out/subtractor-rs

release: build
	# Stage statically-linked binaries
	mkdir -p out/release
	cp out/subtractor-rs out/release/subtractor-rs.linux-$$(uname -m)

install: release
	sudo install out/release/subtractor-rs.linux-$$(uname -m) /usr/local/bin/subtractor-rs

dev:
	while [ -z "$$PID" ] || [ -n "$$(inotifywait -q -r -e modify proto src)" ]; do\
		$(MAKE);\
		kill -9 $$PID 2>/dev/null 1>&2;\
		wait $$PID;\
		out/subtractor-rs & export PID="$$!";\
	done

clean:
	rm -rf out

depend:
	# Install dependencies
	cargo build
