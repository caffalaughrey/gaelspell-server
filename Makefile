IMAGE_NAME    := caffalaughrey/gaelspell
IMAGE_TAG     := latest

.PHONY: all docker-build

all: docker-build

docker-build:
	@echo "Building Docker image $(IMAGE_NAME):$(IMAGE_TAG)…"
	docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .




