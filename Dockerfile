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

# fix Cargo.toml
RUN nix shell -p sed --run 'sed -i 's|site-root = "target/site"|site-root = "site"|' /tmp/nix-store-closure/Cargo.toml'

# Final image is based on scratch. We copy a bunch of Nix dependencies
# but they're fully self-contained so we don't need Nix anymore.
FROM scratch

WORKDIR /app

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
