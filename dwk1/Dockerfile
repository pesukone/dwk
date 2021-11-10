FROM rust:slim-bullseye as builder
WORKDIR app
COPY . .
RUN cargo build --release

FROM rust:slim-bullseye as runtime
WORKDIR app
COPY --from=builder /app/target/release/dwk1 ./
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./dwk1"]
