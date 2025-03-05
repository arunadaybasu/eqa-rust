#!/bin/bash
set -e

echo "Running minimal tests first..."
cargo test --test minimal_test

echo "Running basic math and utility tests..."
cargo test --test basic_tests

echo "Running simple tests..."
cargo test --test simple_tests

echo "Running token tests..."
cargo test --test test_eqa

echo "Running integration tests..."
cargo test --test integration_test

echo "All tests completed successfully!"
