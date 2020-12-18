FROM rustlang/rust:nightly-buster as builder

COPY . /opt/app/
RUN cd /opt/app && cargo build --release

FROM debian:buster-slim as target
RUN apt-get update -qq && apt-get install -y git && rm -rf /var/lib/apt/lists/*
COPY --from=builder /opt/app/target/release/bugs-checker /usr/local/bin/
RUN chmod a+rx /usr/local/bin/bugs-checker && mkdir /opt/app
VOLUME /opt/app
WORKDIR /opt/app
CMD ["/usr/local/bin/bugs-checker"]
