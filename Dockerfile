
FROM rust:latest as build

WORKDIR /build

COPY . .

RUN apt-get update

RUN apt-get -y install python3-pip

RUN pip3 install cargo-lambda

RUN cargo lambda build --release

FROM rust:latest

WORKDIR /function

COPY --from=build /build/target/lambda/rust_lambda/bootstrap .

ENTRYPOINT [ "/function/bootstrap" ]
CMD ["handler.handle_event"]
