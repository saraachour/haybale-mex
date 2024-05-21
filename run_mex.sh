#!/bin/bash

cp ../cacti/cacti.bc cacti.bc
cp ../cacti/cacti.ll cacti.ll
cargo build --bin mex && RUST_LOG=info target/debug/mex 