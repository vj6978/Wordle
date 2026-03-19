# --- Stage 1: Build ---
FROM rust:1.85-slim-bookworm AS builder

# Create a dummy project to cache dependencies
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# copy real source code
COPY src ./src
# Touch main.rs to ensure cargo rebuilds it
RUN touch src/main.rs
RUN cargo build --release

# --- Stage 2: Runtime ---
FROM debian:bookworm-slim

WORKDIR /app
# Copy only the binary from the builder stage
COPY --from=builder /app/target/release/wordle .

# Run the game
ENTRYPOINT ["./wordle"]