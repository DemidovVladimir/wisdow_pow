# Use the official Rust image
FROM rust:latest

# Create a new directory
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Build the application
RUN cargo build --release

# Run the application
CMD ["./target/release/pow-client"]
