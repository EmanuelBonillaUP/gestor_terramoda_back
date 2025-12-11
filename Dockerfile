FROM rust:1.86.0-alpine3.21 AS builder
WORKDIR /app
COPY . .
RUN apk add --no-cache musl-dev \
    && APP_NAME=$(grep '^name' Cargo.toml | awk -F'"' '{print $2}') \
    && echo $APP_NAME > /app/APP_NAME \
    && echo "Building $APP_NAME" \
    && cargo test --release \
    && cargo build --release

FROM alpine:3.21
WORKDIR /app
COPY --from=builder /app/target/release/gestor_terramoda_back /app/gestor_terramoda_back
COPY --from=builder /app/APP_NAME /app/APP_NAME
CMD sh -c "/app/$(cat /app/APP_NAME)"
