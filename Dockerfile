# Use the official Rust image as the base image for building
FROM rust:1.67

# Set the working directory inside the container
WORKDIR /app

# Copy the Rust project files (Cargo.toml and Cargo.lock) to the container
COPY ./Cargo.toml ./Cargo.lock ./

# Build the application without running it
RUN cargo build --release

# Copy the source code to the container
COPY . .

# Build the release version of the application
RUN cargo build --release

# Install sqlx cli
RUN cargo install sqlx-cli

# Set environment variables (if needed)
ENV DATABASE_URL="postgres://{appname}_dev:dev@localhost:5432/{appname}user?sslmode=disable"

# Run SQLx migrations using the SQLx CLI
RUN sqlx migrate run

# Create a minimal runtime image
FROM debian:buster-slim

# Copy the compiled binary from the builder stage to the final image
COPY --from=builder /app/target/release/user /usr/local/bin/user

# Expose the port on which the Actix application will listen
EXPOSE 8080

# Run the Actix application
CMD ["user"]

