FROM rust:1.35.0-slim

RUN apt-get update -y && \
apt-get -y install curl && \
rustup component add clippy

CMD ["/bin/bash"]
