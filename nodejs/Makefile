IMAGE ?= quay.io/brancz/profiling-demo-nodejs:latest

.PHONY: build push all

all: build push

build:
	docker buildx build --platform linux/amd64 -t $(IMAGE) .

push:
	docker buildx build --platform linux/amd64 -t $(IMAGE) --push .
