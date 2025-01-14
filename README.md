# Rust Boilerplate

This is a boilerplate for Rust projects. It includes a basic project structure, a Makefile for common tasks, and a
Dockerfile for building and running the project in a container.


## Getting Started
1. Generate server and client code from the OpenAPI spec:
   ```bash
   rm -rf packages/gen-server && docker compose run --rm openapi-generator-cli
   ```
   
2. Build the project:
   ```bash
   docker compose build app
   ```
   You can build and run:
   ```bash
   docker compose up app --build -d
   ```