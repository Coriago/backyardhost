# Run the project locally
watch $RUST_BACKTRACE="1" $CARGO_PROFILE_DEV_BUILD_OVERRIDE_DEBUG="true":
    dx serve --desktop
