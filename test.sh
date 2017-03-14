set -ex

main() {
    rm -rf tester/tests
    mkdir tester/tests

    cargo run

    # host
    ( cd tester && cargo test )

    # utest
    local targets=(
        # FIXME rust-lang-nursery/compiler-builtins#150
        # thumbv6m-linux-eabi
        thumbv7m-linux-eabi
        thumbv7em-linux-eabi
        thumbv7em-linux-eabihf
    )

    pushd tester
    export export RUST_TARGET_PATH=$(pwd)
    for t in ${targets[@]}; do
        xargo test --target $t --no-run

        for ut in $(find target/$t/debug -executable -maxdepth 1 -name '*-*' ); do
            qemu-arm $ut
        done
    done
    popd

    # arm targets
    local targets=(
        arm-unknown-linux-gnueabi
        armv7-unknown-linux-gnueabihf
    )
    for t in ${targets[@]}; do
        ( cd tester && cargo clean && cross test --target $t )
    done
}

main
