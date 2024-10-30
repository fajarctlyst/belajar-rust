# Stage 1: Builder

FROM rust:slim-bookworm AS builder

# Install musl tools and build dependencies
RUN apt-get update && \
  apt-get install -y \
  pkg-config \
  musl-tools \
  build-essential \
  cmake \
  && rm -rf /var/lib/apt/lists/*

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/app

ENV RUSTFLAGS='-C target-feature=+crt-static -C linker=musl-gcc'

# Enable dependencies to be cached as their own layer by 
# compiling a dummy application
COPY Cargo.toml Cargo.lock* ./

RUN mkdir src && \
  echo "fn main() {}" > src/main.rs && \
  cargo build --release --target x86_64-unknown-linux-musl && \
  rm -rf src/

COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

# Stage 2: Production image

FROM gcr.io/distroless/static-debian12

COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/hello_actix /hello_actix

EXPOSE 8080

USER nonroot:nonroot

ENTRYPOINT ["/hello_actix"]
