CARGO := cargo
PROJECT_NAME := $(shell basename $(CURDIR))
SERVER_BIN := $(PROJECT_NAME)-server
BUILD_DIR := target
GRPC_SERVER := [::1]:50051  # Update with your server address

# Default target: build the project
.PHONY: all
all: build

# Build the project
.PHONY: build
build:
	$(CARGO) build

# Run the project
.PHONY: run
run:
	@echo "Starting the gRPC server..."
	$(CARGO) run --bin $(SERVER_BIN)

# Test the project
.PHONY: test
test:
	$(CARGO) test

# Format the code
.PHONY: fmt
fmt:
	$(CARGO) fmt --all

# Lint the code
.PHONY: lint
lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

# Clean the build artifacts
.PHONY: clean
clean:
	$(CARGO) clean


# Run grpcui to interact with the gRPC server through a UI
.PHONY: ui
ui:
	@command -v grpcui >/dev/null 2>&1 || { \
		echo "Error: grpcui is not installed. You can install following instructions on 'https://github.com/fullstorydev/grpcui"; \
		exit 1; \
	}
	grpcui -plaintext $(GRPC_SERVER)

# grpcurl -plaintext -d '{"name":"youssef" }' '[::1]:50051' helloworld.Greeter.SayHello