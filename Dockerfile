# NixOS Dockefile build env
# https://mitchellh.com/writing/nix-with-dockerfiles

# Get started with nix build env
FROM nixos/nix:latest AS builder

# Copy our source and setup our working dir.
COPY . /tmp/build
WORKDIR /tmp/build

# Build our Nix environment
RUN nix build --verbose --print-build-logs --impure --extra-experimental-features "nix-command flakes" --option filter-syscalls false

# Copy the Nix store closure into a directory. The Nix store closure is the
# entire set of Nix store values that we need for our build.
RUN mkdir /tmp/nix-store-closure
RUN cp -R $(nix-store -qR result/) /tmp/nix-store-closure

RUN ls -la /tmp/nix-store-closure

FROM debian:bookworm-slim as runtime

WORKDIR /app

RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates \
  && apt-get install -y --no-install-recommends wget curl sed \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*

# Copy /nix/store
COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/build/result /app

# Set any required env variables and
ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
EXPOSE 8080

# Run the server
CMD ["/app/farmtasker-au"]
