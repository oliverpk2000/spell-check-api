FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app
COPY --from=builder /app/rsrc/ /rsrc/
COPY --from=builder /app/target/release/spell-check-api .
CMD ["./spell-check-api"]