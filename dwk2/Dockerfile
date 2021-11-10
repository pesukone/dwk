FROM rust:bullseye as builder
WORKDIR app
COPY . .
RUN cargo build --release

FROM rust:slim-bullseye as runtime
WORKDIR app
COPY --from=builder /app/target/release/dwk2 ./
COPY --from=builder /app/templates ./templates
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["./dwk2"]
