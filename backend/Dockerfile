# syntax=docker/dockerfile:1.4
FROM rust:buster AS builder
# ERROR 1: FALSCHER BASE DOCKER => zusammen gucken, was man da machen kann

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY . /code

RUN cargo build --release --offline
RUN ls  -la
RUN cd target/release && ls -la
RUN pwd

FROM debian:buster-slim

EXPOSE 8000
# ERROR 2: --from vergessen
# ERROR 3: falscher path
COPY --from=builder /code/target/release/todos-backend /todos-backend

CMD [ "/todos-backend" ]
