FROM rust:1.56
WORKDIR /app
COPY . .
RUN CARGO_HTTP_MULTIPLEXING=false cargo fetch
RUN cargo install --path .
ENV RUST_LOG info
ENV DATABASE_URL mysql://root:password@localhost:3306/db_name
EXPOSE 3050
CMD ["cota-registry-aggregator"]
