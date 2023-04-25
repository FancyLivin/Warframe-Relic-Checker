#!/bin/bash
npm run --prefix ./front-end/ build
cargo run --manifest-path ./back-end/Cargo.toml