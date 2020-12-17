FROM rustlang/rust:nightly-buster as builder

COPY . /opt/app/
RUN cd /opt/app && cargo build --release --bin sym-counter

FROM debian:buster-slim as target
RUN apt-get update -qq && apt-get install -y git && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/app/target/release/sym-counter /usr/local/bin/
RUN chmod a+rx /usr/local/bin/sym-counter && mkdir /opt/app
VOLUME /opt/app
WORKDIR /opt/app
CMD ["/usr/local/bin/sym-counter"]
