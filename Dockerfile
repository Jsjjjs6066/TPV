# # Use an official Rust image as the build environment
# FROM rust:slim-trixie AS builder
#
# # Set the working directory
# WORKDIR /app
#
# # Copy the Cargo.toml and Cargo.lock files
#
# # Build the dependencies first, this layer will cache until dependencies change
#
# # Copy the source code
# COPY . .
#
# # Build the final release
# RUN cargo build --release
#
# # Use a minimal Alpine image for the final stage
# FROM debian:trixie
#
# # Set the working directory
# WORKDIR /app
#
# # Copy the binary from the builder stage
# COPY --from=builder /app/target/release/TPRL .
# COPY ./index.json .
#
# # Command to run the binary
# # CMD ["ls", "."]
# RUN chmod +x /app/TPRL
# # RUN /app/TPRL index.json
#
# ENV RUST_BACKTRACE=1
# CMD ["/app/TPRL", "./index.json"]
#
# # CMD ["ls", "/app/", "-al"]




# ==========================================
# Stage 1: Common builder for both targets
# ==========================================
FROM rust:slim-trixie AS builder

# Install necessary build tools
RUN apt-get update && apt-get install -y \
    build-essential \
    mingw-w64 \
    pkg-config \
    libssl-dev \
    curl \
    git \
    && rustup target add x86_64-pc-windows-gnu \
    && rustup target add x86_64-unknown-linux-gnu

WORKDIR /app
COPY . .

# Build Linux binary
RUN cargo build --release --target x86_64-unknown-linux-gnu

# Build Windows binary (cross-compiled using mingw)
RUN cargo build --release --target x86_64-pc-windows-gnu

# ==========================================
# Stage 2: Debian 13 runtime image
# ==========================================
FROM debian:trixie AS linux
ENV TERM=xterm
WORKDIR /app
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/TPRL .
COPY ./index.json .
RUN chmod +x /app/TPRL
ENV RUST_BACKTRACE=1
CMD ["/app/TPRL", "./index.json"]

# ==========================================
# Stage 3: Windows 10 runtime image
# ==========================================
FROM dockurr/windows AS windows
WORKDIR /app
COPY --from=builder /app/target/x86_64-pc-windows-gnu/release/TPRL.exe .
COPY ./index.json .
ENV RUST_BACKTRACE=1
CMD ["C:\\app\\TPRL.exe", "C:\\app\\index.json"]

