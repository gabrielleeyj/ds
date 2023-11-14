# Use a Rust base image it already has the necessary tools and libraries like gcc, g++, make, etc.
FROM rust:latest as builder

# Copy the Rust project files
COPY . /app

# Set the working directory
WORKDIR /app

# Build the Rust application
RUN cargo build --release

# Use a smaller base image for the final image
FROM debian:bookworm-slim

# Set the working directory
WORKDIR /app

# Copy the built Rust binary from the builder stage
COPY --from=builder /app/target/release/ds /app/ds

# Install packages
RUN apt-get update && apt-get install -y postgresql-client ca-certificates

# Set up environment variables for the database connection
ENV DATABASE_URL=postgres://username:password@localhost/dsdb

# Expose the port for the Rust application
EXPOSE 8080

# Command to run your Rust application
CMD ["/ds"]

