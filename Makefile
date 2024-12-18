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