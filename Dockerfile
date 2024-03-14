# Builder stage
FROM rust:slim-buster AS builder

WORKDIR /prod

# Copy manifests and the graphql file
COPY Cargo.lock Cargo.toml examples/jsonplaceholder.graphql docker.sh ./

ADD https://github.com/sclevine/yj/releases/download/v5.1.0/yj-linux-amd64 /usr/local/bin/yj
ADD https://github.com/mikefarah/yq/releases/download/v4.40.5/yq_linux_amd64 /usr/local/bin/yq
RUN chmod +x /usr/local/bin/yj /usr/local/bin/yq
RUN chmod +x docker.sh && ./docker.sh

# This is the trick to speed up the building process.
RUN mkdir .cargo \
    && cargo vendor > .cargo/config

# Install required system dependencies and cleanup in the same layer
RUN apt-get update && apt-get install -y pkg-config libssl-dev python g++ git make libglib2.0-dev && apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy the rest of the source code
COPY . .

# Compile the project
RUN RUST_BACKTRACE=1 cargo build --release

# Runner stage
FROM fedora:41 AS runner

# Copy necessary files from the builder stage
COPY --from=builder /prod/target/release/tailcall /bin
COPY --from=builder /prod/jsonplaceholder.graphql /jsonplaceholder.graphql

# Install tailcall globally (latest version)
RUN docker pull ghcr.io/tailcallhq/tailcall/tc-server
RUN docker run -p 8080:8080 -p 8081:8081 ghcr.io/tailcallhq/tailcall/tc-server

ENV TAILCALL_LOG_LEVEL=error
CMD ["/bin/tailcall", "start", "jsonplaceholder.graphql"]