FROM rust:alpine AS build

RUN apk add clang musl-dev pkgconfig

WORKDIR /usr/src/laguna-backend
COPY . .
RUN cargo install sqlx-cli --no-default-features --features rustls,postgres
RUN cargo build --release

FROM alpine

ENV DATABASE_URL=postgres://postgres:postgres@host.docker.internal:5432/laguna_db
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=6969
ENV FRONTEND_HOST=0.0.0.0
ENV FRONTEND_PORT=4200

COPY --from=build /usr/src/laguna-backend/target/release/laguna-backend /usr/local/bin/laguna
COPY --from=build /usr/src/laguna-backend/migrations /opt/laguna/migrations
COPY --from=build /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

WORKDIR /opt/laguna
CMD sqlx database setup --database-url=${DATABASE_URL} && laguna
