FROM rust:1.40 as builder

WORKDIR /medidor
COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /medidor
COPY --from=builder /medidor .
RUN mv target/release/medidor .

CMD [ "medidor" ]