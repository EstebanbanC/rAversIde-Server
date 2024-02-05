# Use the official Rust image
FROM rust:latest as builder

# Configurer Rust pour utiliser la version nightly
RUN rustup default nightly

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code to the working directory
COPY . .

# Build the Rust project in release mode
RUN cargo build --release

# Set Rocket to listen on all network interfaces
ENV ROCKET_ADDRESS=0.0.0.0

# Expose both the HTTP port and the WebSocket port
EXPOSE 8000
EXPOSE 8080

CMD ["./target/release/serveur"]

