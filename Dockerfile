FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin believex-backend

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/believex-backend .
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/models ./models

EXPOSE 7331
ENV RUST_LOG info

ENTRYPOINT [ "./believex-backend" ]
