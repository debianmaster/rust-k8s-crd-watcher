
FROM alpine:latest as builder
# Choose a workdir
WORKDIR /usr/src/app
# Create blank project
RUN apk add --no-cache openssl-dev  rust cargo  && USER=root cargo init
# Copy Cargo.toml to get dependencies
COPY Cargo.toml .
# This is a dummy build to get the dependencies cached
RUN PKG_CONFIG_ALLOW_CROSS=1 cargo build --release
# Copy sources
COPY src src
RUN cargo build --release

FROM alpine:3.11
RUN apk add --no-cache libgcc
# Copy bin from builder to this new image
COPY --from=builder /usr/src/app/target/release/rust-k8s-crd-watcher /rust-k8s-crd-watcher
# Default command, run app
ENTRYPOINT [ "/rust-k8s-crd-watcher" ] 



