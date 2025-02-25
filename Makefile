CARGO := cargo
PROJECT_NAME := $(shell basename $(CURDIR))
SERVER_BIN := $(PROJECT_NAME)-server
BUILD_DIR := target
GRPC_SERVER := [::1]:50051

.PHONY: build
build:
	$(CARGO) build

.PHONY: run
run:
	@echo "Starting the gRPC server..."
	$(CARGO) run --bin $(SERVER_BIN)

.PHONY: test
test:
	$(CARGO) test

.PHONY: fmt-check
fmt-check: check fmt

check:
	$(CARGO) fmt --package vulnerabilities -- --check

.PHONY: fmt
fmt:
	$(CARGO) fmt --package vulnerabilities

.PHONY: lint
lint:
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: generate
generate_dp_track:
	@echo "Generating Rust code from the OpenAPI specification..."
	@openapi-generator generate \
        -i dtrack.json \
        -g rust \
        -o dependencytrack \
        --package-name dependencytrack || { \
            echo "Error: openapi-generator is not installed or failed to execute."; \
            echo "Please visit https://openapi-generator.tech/docs/installation/ for installation instructions."; \
            exit 1; \
        }