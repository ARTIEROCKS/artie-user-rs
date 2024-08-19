# Use the official Rust image as the base for the builder stage
FROM rust:1.71 as builder

# Install protoc (Protocol Buffers compiler)
RUN apt-get update && apt-get install -y protobuf-compiler

# Set the working directory
WORKDIR /usr/src/user_service

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty directory for build dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN echo "fn main() {}" > src/lib.rs

# Compile the dependencies
RUN cargo build --release --bin user_service
RUN rm -r src

# Copy the rest of the project files
COPY . .

# Compile the project
RUN cargo build --release --bin user_service

# Create a new stage for a runtime image with necessary libraries
FROM ubuntu:20.04

# Install necessary dependencies to run the binary
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/user_service/target/release/user_service .

# Expose the port where the gRPC service will be listening
EXPOSE 50051

# Set environment variables
ENV RUST_LOG=debug

# Command to run the binary
CMD ["./user_service"]
