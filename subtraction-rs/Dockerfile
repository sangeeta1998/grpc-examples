# Build container
FROM debian:bullseye AS build

# Setup environment
RUN mkdir -p /data
WORKDIR /data

# Build the release
COPY . .
RUN ./Hydrunfile

# Extract the release
RUN mkdir -p /out
#RUN cp out/release/subtraction-rs.linux-$(uname -m) /out/subtraction-rs
#Copied the binary file name from /out/release 
RUN cp out/release/subtractor-rs.linux-x86_64 /out/subtraction-rs  


# Release container
FROM debian:bullseye

# Add certificates
RUN apt update
RUN apt install -y ca-certificates

# Add the release
COPY --from=build /out/subtraction-rs /usr/local/bin/subtraction-rs

CMD /usr/local/bin/subtraction-rs
