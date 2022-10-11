FROM rust:1.63.0 as build
ENV PKG_CONFIG_ALLOW_CROSS=1
ARG DATABASE_URL
WORKDIR /usr/src/api-service
COPY . .
ENV DATABASE_URL=$DATABASE_URL SQLX_OFFLINE=true RUST_BACKTRACE=1
EXPOSE 8080
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian10

COPY --from=build /usr/local/cargo/bin/hexa-domain-tutorial /usr/local/bin/hexa-domain-tutorial

ENTRYPOINT ["/usr/local/bin/hexa-domain-tutorial"]