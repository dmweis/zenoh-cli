TARGET_HOST ?= homepi
TARGET_USERNAME ?= pi
TARGET_HOST_USER ?= $(TARGET_USERNAME)@$(TARGET_HOST)

DEB_BUILD_PATH ?= target/debian/zenoh-cli*.deb

.PHONY: build
build:
	cargo build --release

.PHONY: install-cargo
install-cargo:
	cargo install --path .

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
	rm -rf docker_out
	mkdir docker_out
	DOCKER_BUILDKIT=1 docker build --tag zenoh-cli-builder --file Dockerfile --output type=local,dest=docker_out .
	
.PHONY: push-docker-built
push-docker-built: build-docker
	rsync -avz --delete docker_out/*.deb $(TARGET_HOST_USER):/home/$(TARGET_USERNAME)/zenoh-cli-built/

.PHONY: gh-create-release
gh-create-release:
	gh release create v$$(cargo get package.version) --title v$$(cargo get package.version) --notes ""

.PHONY: gh-version-exists
gh-version-exists:
	gh release list  --json name | jq '.[].name' | rg -q "v$$(cargo get package.version)" && echo "Version found" || echo "Version not found"

.PHONY: gh-upload-arm64
gh-upload-arm64: build-docker
	gh release upload v$$(cargo get package.version) --clobber docker_out/zenoh-cli-arm64.deb docker_out/zenoh-cli_*.deb
	@echo deb image at https://github.com/dmweis/zenoh-cli/releases/latest/download/zenoh-cli-arm64.deb

.PHONY: gh-upload-amd64
gh-upload-amd64: build-deb
	cp target/debian/zenoh-cli_*amd64.deb tmp/zenoh-cli-amd64.deb
	gh release upload v$$(cargo get package.version) --clobber tmp/zenoh-cli-amd64.deb target/debian/zenoh-cli_*amd64.deb 
	@echo deb image at https://github.com/dmweis/zenoh-cli/releases/latest/download/zenoh-cli-amd64.deb
