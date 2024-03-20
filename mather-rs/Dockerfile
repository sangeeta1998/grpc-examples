# Build container
FROM rust:latest AS build

# Install required packages
RUN apt-get update && apt-get install -y \
    curl \
    make \
    && rm -rf /var/lib/apt/lists/*

# Setup environment
RUN mkdir -p /data
WORKDIR /data

# Copy the source code into the container
COPY . .

# Build the release
RUN ./Hydrunfile

# Extract the release
RUN mkdir -p /out
RUN cp out/release/mather-rs.linux-$(uname -m) /out/mather-rs

# Release container
FROM debian:bullseye

# Add certificates
RUN apt update && apt install -y ca-certificates

# Copy the release binary from the build stage
COPY --from=build /out/mather-rs /usr/local/bin/mather-rs

CMD /usr/local/bin/mather-rs

