FROM rust:1.63.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1
ARG DATABASE_URL
WORKDIR /usr/src/api-service
COPY . .
ENV DATABASE_URL=$DATABASE_URL SQLX_OFFLINE=true
#ENV
ENV RUST_BACKTRACE=1
EXPOSE 8080
COPY sqlx-data.json .
#RUN cargo install sqlx-cli
#RUN cargo install sqlx-cli --no-default-features --features postgres
#RUN cargo sqlx prepare  --database-url $DATABASE_URL
#RUN cargo sqlx prepare  --database-url 'postgres://postgres:somePassword@postgres:5432/postgres'
RUN cargo install --path .
#RUN cargo install sqlx-cli && cargo sqlx prepare --check */&& cargo install --path .

FROM gcr.io/distroless/cc-debian10


ENV  DATABASE_URL=$DATABASE_URL
ENV SQLX_OFFLINE=true
ENV RUST_BACKTRACE=1

COPY --from=build /usr/local/cargo/bin/hexa-domain-tutorial /usr/local/bin/hexa-domain-tutorial

ENTRYPOINT ["/usr/local/bin/hexa-domain-tutorial"]