# Sample configuration taken from leptos
# Get started with a build env with Rust nightly
FROM rust:alpine as builder

#Install libc equivalent
RUN apk add --no-cache musl-dev
RUN apk add --no-cache pkgconfig
RUN apk add --no-cache libressl-dev

# Make an /app dir, which everything will eventually live in
RUN mkdir -p /app
WORKDIR /app
COPY . .
WORKDIR /app/backend

# Build the app
RUN cargo build --release

RUN cp ./target/release/backend /bin/server

FROM alpine as runner
# Copy the server binary to the /app directory
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=builder /bin/server /bin/

# Set any required env variables and
ENV RUST_LOG="info"
EXPOSE 6942

CMD ["/bin/backend"]
