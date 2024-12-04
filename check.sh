#!/bin/bash

# Color codes
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Timeout settings
TIMEOUT_DURATION=300  # 5 minutes

# Comprehensive project health check script

# Function to print section header
print_section() {
    echo -e "${YELLOW}$1${NC}"
}

# Function to print success
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Function to print error
print_error() {
    echo -e "${RED}✗ $1${NC}"
}

# Function to run command with timeout
run_with_timeout() {
    timeout $TIMEOUT_DURATION "$@"
}

# Main check script
main() {
    echo -e "${YELLOW}Running Rust project checks...${NC}"

    # Formatting check
    print_section "Checking formatting..."
    run_with_timeout cargo fmt
    if [ $? -eq 0 ]; then
        print_success "Formatting passed"
    else
        print_error "Formatting failed"
        exit 1
    fi

    # Clippy linting
    print_section "Running Clippy linter..."
    run_with_timeout cargo clippy -- -D warnings
    if [ $? -eq 0 ]; then
        print_success "Clippy Linting passed"
    else
        print_error "Clippy Linting failed"
        exit 1
    fi

    # Run test suite
    print_section "Running test suite..."
    run_with_timeout cargo test
    if [ $? -eq 0 ]; then
        print_success "Tests passed"
    else
        print_error "Tests failed"
        exit 1
    fi

    # Skipping dependency check
    # print_section "Checking dependencies..."
    # run_with_timeout cargo outdated
    # if [ $? -eq 0 ]; then
    #     print_success "Dependencies up to date"
    # else
    #     print_error "Some dependencies are outdated"
    #     exit 1
    # fi
}

# Run main function
main

    # Final success message
    echo -e "${GREEN}✓ All checks passed successfully!${NC}"
