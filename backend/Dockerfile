# syntax=docker/dockerfile:1.4
# Base images ist ein image, in welchem der Rust Compiler schon installiert ist
FROM rust:buster AS builder

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
# Lädt Abhängigkeiten runter
RUN cargo fetch
# Kopiert den Code vom host ins image
COPY . /code

# Compiliert eine executable mit dem namen des Projektes (siehe Cargo.toml) in den Ordner
# ./target/release/project_name
RUN cargo build --release --offline

# Image, das die executable dann ausführt, ohne dass der Rust Compiler installiert sein muss
FROM debian:buster-slim

# Erlaubt Docker mit diesem Port zu interagieren
EXPOSE 8000

# Kopiert die executable ins Image
COPY --from=builder /code/target/release/todos-backend /todos-backend

ENV RUST_LOG=info

# Führt die executable aus
CMD [ "/todos-backend" ]
