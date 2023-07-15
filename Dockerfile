FROM rust:alpine AS build

RUN apk add clang musl-dev

WORKDIR /usr/src/laguna-backend
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11

ENV DATABASE_URL=postgres://postgres:postgres@host.docker.internal:5432/laguna_db
ENV RUST_LOG=info
ENV HOST=0.0.0.0
ENV PORT=6969
ENV FRONTEND_HOST=0.0.0.0
ENV FRONTEND_PORT=4200

COPY --from=build /usr/src/laguna-backend/target/release/laguna-backend /usr/local/bin/laguna

WORKDIR /usr/local/bin
CMD ["laguna"]
