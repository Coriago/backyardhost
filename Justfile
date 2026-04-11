# Run the project locally
download-nats:
    [ -f target/nats-server ] || (mkdir -p target && curl -fsSL https://github.com/nats-io/nats-server/releases/download/v2.11.3/nats-server-v2.11.3-linux-amd64.tar.gz | tar -xz -C target && mv target/nats-server-v2.11.3-linux-amd64/nats-server target/nats-server && chmod +x target/nats-server && rm -rf target/nats-server-v2.11.3-linux-amd64)

watch $RUST_BACKTRACE="1" $CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG="true": download-nats
    dx serve --web
