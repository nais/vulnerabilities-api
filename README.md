# Vulnerabilities-api

This project is a gRPC service that provides information about vulnerabilities in the Nais platform. It consolidates
data from various sources and offers a unified interface for querying vulnerability information.

## Vulnerability sources

* Dependency-Track
* (More sources to come)

## Getting Started

### Prerequisites

Ensure you have the required dependencies installed (e.g., Rust, openapi-generator if needed).
Set up a .env file in your local environment. You can use .env.example as a template.

#### Building the Project

To build the project, run:

```shell
make build
```

#### Running the Server

To start the gRPC server, execute:

```shell
make run
```

The server will start, ready to process requests related to vulnerability data.

## OpenAPI Generated Code

The code for interacting with the Dependency-Track API is auto-generated.
To regenerate this code, use the following command:

```shell
make generate_dp_track
```

This will generate Rust code and place it in the `dependencytrack` folder.

## Common Issues with Generated Code

### Known Flaw

The OpenAPI specification might not always align perfectly with the server responses. This mismatch can result in flawed
generated code.

### How to Fix

1. Update the OpenAPI specification (dtrack.json) to match the server's actual response.
2. Regenerate the code by running:

```shell
make generate_dp_track
```
