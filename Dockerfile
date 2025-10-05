# Use an official Rust image as the build environment
FROM rust:slim-trixie AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files

# Build the dependencies first, this layer will cache until dependencies change

# Copy the source code
COPY . .

# Build the final release
RUN cargo build --release

# Use a minimal Alpine image for the final stage
FROM debian:trixie

# Set the working directory
WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/TPRL .
COPY ./index.json .

# Command to run the binary
# CMD ["ls", "."]
RUN chmod +x /app/TPRL
# RUN /app/TPRL index.json

ENV RUST_BACKTRACE=1
CMD ["/app/TPRL", "./index.json"]

# CMD ["ls", "/app/", "-al"]
