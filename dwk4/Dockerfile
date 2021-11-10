FROM rust:slim-bullseye as builder
WORKDIR app
COPY . .
RUN cargo build --release

FROM rust:slim-bullseye as runtime
WORKDIR app
COPY --from=builder /app/target/release/dwk4 ./
CMD ["./dwk4"]
