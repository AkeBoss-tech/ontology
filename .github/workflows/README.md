# GitHub Actions Workflows

## Rust Workflow (`rust.yml`)

This workflow runs comprehensive tests and builds for the Rust codebase.

### Jobs

1. **test** - Runs unit and integration tests (without external services)
   - Matrix strategy for unit vs integration tests
   - Format checking with `cargo fmt`
   - Linting with `cargo clippy`
   - Builds all workspace packages
   - Runs all test suites

2. **test-with-services** - Runs integration tests with Elasticsearch and Dgraph
   - Only runs on pull requests and main branch
   - Sets up Elasticsearch 8.11.0
   - Sets up Dgraph standalone v23.1.0
   - Waits for services to be ready
   - Runs store and integration tests

3. **lint** - Standalone linting job
   - Format checking
   - Clippy warnings as errors

4. **build** - Release builds
   - Builds all packages in release mode
   - Builds GraphQL server binary
   - Uploads build artifacts

### Features

- ✅ Cargo caching for faster builds
- ✅ Matrix testing strategy
- ✅ Service containers for integration tests
- ✅ Artifact uploads
- ✅ Comprehensive test coverage

### Running Locally

To run the same tests locally:

```bash
# Unit tests
cargo test --lib --workspace
cargo test --test unit_test --package indexing

# Integration tests (requires services)
ELASTICSEARCH_URL=http://localhost:9200 DGRAPH_URL=http://localhost:9080 \
  cargo test --test store_test --package indexing

# Format check
cargo fmt --all -- --check

# Clippy
cargo clippy --all-targets --all-features -- -D warnings
```

