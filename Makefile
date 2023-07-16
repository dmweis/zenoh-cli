DEB_BUILD_PATH ?= target/debian/zenoh-cli*.deb

.PHONY: build
build:
	cargo build --release

.PHONY: build-deb
build-deb: build
	cargo deb --no-build

.PHONE: install
install: build-deb
	sudo dpkg -i $(DEB_BUILD_PATH)

.PHONY: install-deps
install-deps:
	cargo install cargo-deb

.PHONY: build-docker
build-docker:
	DOCKER_BUILDKIT=1 docker build --tag zenoh-cli-builder --file Dockerfile --output type=local,dest=docker_out .
