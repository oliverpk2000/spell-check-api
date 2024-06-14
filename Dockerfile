FROM rust:latest as builder
WORKDIR /app
COPY . .
CMD ["cargo", "build", "--release"]

FROM rust:latest
WORKDIR /app
COPY --from=builder /app /app
CMD ["cargo","run","--release"]