#!/bin/bash

# Set color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓ $2 passed${NC}"
    else
        echo -e "${RED}✗ $2 failed${NC}"
    fi
}

# Timeout function
run_with_timeout() {
    local timeout=$1
    shift
    timeout $timeout "$@"
}

# Main script
echo -e "${YELLOW}Running Rust project checks...${NC}"

# 1. Formatting check
echo -e "\n${YELLOW}Checking formatting...${NC}"
run_with_timeout 5 cargo fmt 
FMT_STATUS=$?
print_status $FMT_STATUS "Formatting"

# 2. Clippy linting
echo -e "\n${YELLOW}Running Clippy linter...${NC}"
run_with_timeout 5 cargo clippy -- -D warnings
CLIPPY_STATUS=$?
print_status $CLIPPY_STATUS "Clippy Linting"

# 3. Run tests
echo -e "\n${YELLOW}Running test suite...${NC}"
run_with_timeout 5 cargo test
TEST_STATUS=$?
print_status $TEST_STATUS "Tests"

# Overall status
if [ $FMT_STATUS -eq 0 ] && [ $CLIPPY_STATUS -eq 0 ] && [ $TEST_STATUS -eq 0 ]; then
    echo -e "\n${GREEN}✓ All checks passed successfully!${NC}"
    exit 0
else
    echo -e "\n${RED}✗ Some checks failed. Please review the output.${NC}"
    exit 1
fi
