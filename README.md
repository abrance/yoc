# yoc

yoc is a GitHub reusable workflow repository. It provides stable, documented workflow interfaces for common CI/CD tasks so downstream repositories can call shared automation instead of copying YAML.

## Workflows

| Workflow | Purpose |
| --- | --- |
| `.github/workflows/rust-ci.yml` | Run Rust format, clippy, tests, and build. |
| `.github/workflows/go-ci.yml` | Run Go dependency download, vet, tests, and build. |
| `.github/workflows/react-ci.yml` | Run React install, lint, typecheck, tests, and build. |
| `.github/workflows/vue-ci.yml` | Run Vue install, lint, typecheck, tests, and build. |
| `.github/workflows/docker-build.yml` | Build Docker images for validation without publishing by default. |
| `.github/workflows/docker-publish-ghcr.yml` | Build and publish Docker images to GitHub Container Registry. |

## Quick Start

Call a workflow from another repository:

```yaml
name: CI

on:
  pull_request:
  push:
    branches: [main]

jobs:
  rust-ci:
    uses: owner/yoc/.github/workflows/rust-ci.yml@v1
    with:
      working-directory: .
      rust-toolchain: stable
```

Use a stable major version tag such as `@v1` for normal projects. Use an exact version such as `@v1.2.3` when reproducibility matters more than automatic compatible updates.

## Documentation

- [Usage](docs/usage.md)
- [Versioning](docs/versioning.md)
- [Security](docs/security.md)

## Self Test

The repository CI at `.github/workflows/ci.yml` runs on pull requests and pushes to `main`. It calls the local reusable workflows against the minimal projects under `fixtures/`, so changes to workflow contracts are tested before release.

## Design Rules

- Workflows expose `workflow_call` interfaces.
- Inputs, secrets, permissions, and outputs are explicit.
- Defaults are safe for pull requests.
- Publishing workflows request write permissions only where needed.
- Project-specific paths and image names belong in caller repositories, not in yoc.
