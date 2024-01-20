# Sample configuration taken from leptos
# Get started with a build env with Rust nightly
FROM rustlang/rust:alpine as builder

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

FROM alpine as runner
# Copy the server binary to the /app directory
COPY --from=builder /app/backend/target/release/backend /app/

WORKDIR /app
# Set any required env variables and
ENV RUST_LOG="info"
EXPOSE 6942
# Run the server
CMD ["/app/backend"]
