@echo off
echo Cleaning project and running tests...

:: Kill any processes that might be holding onto files
taskkill /F /IM collateral_cross_chain_test-*.exe 2>nul
taskkill /F /IM rust.exe 2>nul
taskkill /F /IM rustc.exe 2>nul

:: Clean the target directory
cargo clean 

:: Pause to allow filesystem to catch up
timeout /t 3

:: Run specific tests first, avoiding problematic ones
echo Running simplified tests...
cargo test --test simplified_tests

echo Running basic tests...
cargo test --test basic_tests

:: Run the rest of the tests
echo Running remaining tests...
cargo test --test cross_chain_test
cargo test --test collateral_registry_test

echo All tests completed
