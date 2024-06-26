# mather-rs

Simple Rust gRPC microservice that does math.

[![Docker Pulls](https://img.shields.io/docker/pulls/pojntfx/mather-rs?label=docker%20pulls)](https://hub.docker.com/r/pojntfx/mather-rs)
[![Binary Downloads](https://img.shields.io/github/downloads/pojntfx/grpc-examples/latest/mather-rs.linux-x86_64?label=binary%20downloads)](https://github.com/pojntfx/grpc-examples/releases)

> Be sure to also check out the [other gRPC examples](../README.md) for more information on how to contribute and more!

## Installation

### Kubernetes

You can add the chart repo like so:

```shell
$ helm repo add pojntfx https://pojntfx.github.io/charts/
```

### Containerized

You can get the Docker container like so:

```shell
$ docker pull ghcr.io/pojntfx/mather-rs
```

### Natively

If you prefer a native installation, binaries are also available on [GitHub releases](https://github.com/pojntfx/grpc-examples/releases).

You can install them like so:

```shell
$ curl -L -o /tmp/mather-rs https://github.com/pojntfx/grpc-examples/releases/latest/download/mather-rs.linux-$(uname -m)
$ sudo install /tmp/mather-rs /usr/local/bin
```

## Usage

### 1 (Option 1): Starting the Microservice (Kubernetes)

Helm is the easiest option to start the microservice on Kubernetes.

<details>
  <summary>Expand installation instructions</summary>

Run the following; see the [Reference](#reference) for more configuration parameters:

```shell
$ helm install mather-rs pojntfx/mather-rs --set app.multiplier=1
```

The logs are available like so:

```shell
$ kubectl logs mather-rs
```

  </details>

### 1 (Option 2): Starting the Microservice (Containerized)

Using Docker (or an alternative like Podman) is also possible.

<details>
  <summary>Expand installation instructions</summary>

Run the following; see the [Reference](#reference) for more configuration parameters:

```shell
$ docker run \
    --name mather-rs \
    -d \
    --restart always \
    -p 5000:5000 \
    -e MULTIPLIER=1 \
    ghcr.io/pojntfx/mather-rs
```

The logs are available like so:

```shell
$ docker logs mather-rs
```

  </details>

### 1 (Option 3): Starting the Microservice (Natively)

If you prefer a native setup, a non-containerized installation is also possible.

<details>
  <summary>Expand installation instructions</summary>

First, create a systemd service for it; see the [Reference](#reference) for more configuration parameters::

```shell
$ mkdir -p ~/.config/systemd/user/
$ cat <<EOT >~/.config/systemd/user/mather-rs.service
[Unit]
Description=mather-rs

[Service]
Environment="MULTIPLIER=1"
ExecStart=/usr/local/bin/mather-rs

[Install]
WantedBy=multi-user.target
EOT
```

Finally, reload systemd and enable the service:

```shell
$ systemctl --user daemon-reload
$ systemctl --user enable --now mather-rs
```

You can get the logs like so:

```shell
$ journalctl --user -u mather-rs
```

  </details>

### 2. Making a Request

Now that the microservice is running, run a request to test it:

```shell
grpcurl --plaintext --proto proto/mather.proto -d '{"FirstSummand": 1, "SecondSummand": 3}' localhost:5000 com.pojtinger.felicitas.grpcExamples.Mather.Add
```


### ADDED ONE MORE SERVICE:


# Navigate to mather-rs directory
cd mather-rs

# Build the mather-rs Docker image
docker build -t mather-rs .

# Run the mather-rs container
docker run -d --name mather-rs -p 5000:5000 mather-rs
(If it doesn't work), try :
docker run --name mather-rs -d --restart always -p 5000:5000 -e MULTIPLIER=1 ghcr.io/pojntfx/mather-rs


# Verify if service is running

docker logs mather-rs
docker logs subtraction-rs

# Using Docker compose
 
docker-compose build
docker-compose up -d
# Cleaning up
docker-compose down

# Testing mather-rs
grpcurl --plaintext --proto mather-rs/proto/mather.proto -d '{"FirstSummand": 1, "SecondSummand": 3}' localhost:5000 com.pojtinger.felicitas.grpcExamples.Mather.Add

# Testing subtraction-rs
grpcurl --plaintext --proto subtraction-rs/proto/subtractor.proto -d '{"Minuend": 10, "Subtrahend": 5}' localhost:5001 com.pojtinger.felicitas.grpcExamples.Subtractor.Subtract

# Calling subtraction-rs from mather-rs
grpcurl -plaintext -d '{"FirstSummand": 10, "SecondSummand": 5}' -import-path ~/grpc-examples/subtraction-rs/proto/ -proto subtractor.proto localhost:5001 com.pojtinger.felicitas.grpcExamples.Mather/Add





🚀 **That's it**! We hope you enjoy using mather-rs.

## Reference

### Environment Variables

You can set the multiplier, which multiplies each sum, using the `MULTIPLIER` environment variable.

If you're on Kubernetes, also check out the available [Helm chart values](./charts/mather-rs/values.yaml) which you can use to adjust available resources, set the domain name and more.

### gRPC API

mather-rs exposes a gRPC API. You can find the relevant `.proto` files in [proto](./proto).

## License

mather-rs (c) 2021 Felicitas Pojtinger and contributors

SPDX-License-Identifier: AGPL-3.0
