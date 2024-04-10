# Use a Rust base image for building
FROM rust:latest AS builder

# Set working directory
WORKDIR /usr/src/app

# Install required tools for building WebAssembly
RUN apt-get update && \
    apt-get install -y \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Copy your Rust code
COPY . .

# Build the WebAssembly binary
RUN wasm-pack build --target web --out-dir ./target/web

# Build container for serving WebAssembly
FROM nginx:alpine

# Set working directory
WORKDIR /usr/share/nginx/html

# Copy the WebAssembly files
COPY --from=builder /usr/src/app/target/web .

# Expose port
EXPOSE 80

# Start nginx to serve the WebAssembly files
CMD ["nginx", "-g", "daemon off;"]

