.PHONY: build dev dev-down

## build — compile sl-daemon and sl-viewer (debug profile)
build:
	cargo build -p sl-daemon -p sl-viewer

## dev — build both crates then bring up the process-compose stack
dev: build
	process-compose up

## dev-down — tear down the process-compose stack
dev-down:
	process-compose down
