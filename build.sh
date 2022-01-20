#!/bin/bash
rustc --crate-type "cdylib" --target wasm32-unknown-unknown bones.rs
