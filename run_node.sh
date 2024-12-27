RUST_LOG=info pba-omni-node \
    --runtime ./target/release/wbuild/minimal-template-runtime/minimal_template_runtime.wasm \
    --tmp \
    --consensus manual-seal-5000 \
    --offchain-worker always
