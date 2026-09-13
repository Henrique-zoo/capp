FROM rust:1-bookworm

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests

RUN cargo fetch

CMD ["cargo", "run"]
