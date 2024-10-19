# Build state
FROM rust:1.70-buster as builder

WORKDIR /app

# Accept the build argument
ARG DATABASE_URL

# Make sure to use the ARG in ENV
ENV DATABASE_URL=$DATABASE_URL

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Production state
FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/RustAPI .

CMD ["./RustAPI"]