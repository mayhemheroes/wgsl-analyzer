# Build Stage
FROM ghcr.io/evanrichter/cargo-fuzz:latest as builder

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd crates/wgsl_parser/fuzz && cargo +nightly -Z sparse-registry fuzz build && popd" && \
    mv crates/wgsl_parser/fuzz/target/x86_64-unknown-linux-gnu/release/wgsl_parse_file /wgsl_parse_file && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd crates/wgsl_parser/fuzz && cargo +nightly -Z sparse-registry build --release && popd" && \
    mv crates/wgsl_parser/fuzz/target/release/wgsl_parse_file /wgsl_parse_file_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /wgsl_parse_file /wgsl_parse_file_no_inst /
