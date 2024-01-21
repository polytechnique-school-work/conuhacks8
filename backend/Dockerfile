# Sample configuration taken from leptos
# Get started with a build env with Rust nightly
FROM rust:alpine as builder

#Install libc equivalent
RUN apk add --no-cache musl-dev
RUN apk add --no-cache pkgconfig
RUN apk add --no-cache libressl-dev

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --bin backend --release
cp ./target/release/backend /bin/backend
EOF

FROM alpine as runner
# Copy the server binary to the /app directory

COPY --from=builder /bin/backend /bin/

COPY ./datafile.csv datafile.csv

# Set any required env variables and
ENV RUST_LOG="debug"
EXPOSE 6942

CMD ["/bin/backend"]
