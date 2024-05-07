FROM rust AS build_host
WORKDIR /src

RUN USER=root cargo new --bin users
WORKDIR /src/users

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs
RUN rm ./target/release/deps/users*

COPY ./src ./src
RUN cargo build --release

WORKDIR /src

FROM rust:slim

RUN apt-get update
RUN apt-get install -y libpq-dev

WORKDIR /src

COPY --from=build_host /src/users/target/release/users ./users

CMD ["./users"]
