FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

CMD ["./target/release/ci_basic_github_rust"]
