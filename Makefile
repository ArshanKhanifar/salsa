.PHONY: build run build-parallel install-parallel publish

define select_project
	if [ -z "$(project)" ]; then \
		export project=`ls crates | fzf --height 40% --reverse`; \
	else \
		export project=`ls crates | grep -i $(project) | head -n 1`; \
	fi; \
	if [ -z "$$project" ]; then \
		echo "No project selected"; \
		exit 1; \
	fi; \
	echo "Selected project: $$project"
endef
export select_project

build:
	@eval "$$select_project"; \
	cargo build -p $$project

run:
	@eval "$$select_project"; \
	cargo run -p $$project

# Build parallel binary for current architecture
build-parallel:
	cargo build --release -p parallel
	@mkdir -p dist
	@cp target/release/parallel dist/parallel-$(shell uname -m)

# Install parallel binary
install-parallel: build-parallel
	@echo "Installing parallel binary..."
	@sudo cp dist/parallel-$(shell uname -m) /usr/local/bin/parallel
	@echo "Installed to /usr/local/bin/parallel"

# Build parallel for both architectures (using zig)
build-parallel-all:
	@mkdir -p dist
	@echo "Installing cargo-zigbuild..."
	cargo install cargo-zigbuild
	@echo "Installing Zig..."
	brew install zig
	@echo "Building for arm64..."
	cargo zigbuild --release --target aarch64-unknown-linux-gnu -p parallel
	@cp target/aarch64-unknown-linux-gnu/release/parallel dist/parallel-arm64
	@echo "Building for amd64..."
	cargo zigbuild --release --target x86_64-unknown-linux-gnu -p parallel
	@cp target/x86_64-unknown-linux-gnu/release/parallel dist/parallel-amd64

# Create a new release tag and push it
publish:
	@echo "Creating release tag..."
	$(eval COMMIT_HASH := $(shell git rev-parse --short HEAD))
	$(eval VERSION := v0.1.0-$(COMMIT_HASH))
	@echo "Tagging as $(VERSION)..."
	@git tag -a $(VERSION) -m "Release $(VERSION)"
	@echo "Pushing tag..."
	@git push origin $(VERSION)
	@echo "Release $(VERSION) has been tagged and pushed."
	@echo "GitHub Actions will now build and publish the release."

# Help command
help:
	@echo "Available commands:"
	@echo "  make build [project=<n>]  - Build a project (uses fzf if project not specified)"
	@echo "  make run [project=<n>]    - Run a project (uses fzf if project not specified)"
	@echo "  make build-parallel       - Build parallel binary for current architecture"
	@echo "  make install-parallel     - Install parallel binary to /usr/local/bin"
	@echo "  make build-parallel-all   - Build parallel binary for both arm64 and amd64"
	@echo "  make publish             - Create and push a new release tag"
	@echo "\nAvailable projects:"
	@ls crates
