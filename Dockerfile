FROM rust:slim as builder
WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

FROM rust:slim
WORKDIR /usr/src/myapp

RUN apt-get update -y \
        && apt-get upgrade -y \
        && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/actix .
COPY --from=builder /usr/src/myapp .

CMD ["./actix"]