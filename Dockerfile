FROM debian:bookworm-slim
WORKDIR /app
COPY ./target/release/zero_to_production zero_to_production
COPY configuration configuration
ENTRYPOINT ["/app/zero_to_production"]
