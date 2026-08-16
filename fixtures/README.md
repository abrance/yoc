# Workflow Test Fixtures

These minimal projects exercise yoc reusable workflows without coupling them to a real product repository.

| Fixture | Workflow | Purpose |
| --- | --- | --- |
| `go-hello` | `.github/workflows/go-ci.yml` | Small Go HTTP hello server with a handler test. |
| `rust-hello` | `.github/workflows/rust-ci.yml` | Small Rust HTTP hello server with a response test. |
| `react-hello` | `.github/workflows/react-ci.yml` | Small Vite React TypeScript UI. |
| `vue-hello` | `.github/workflows/vue-ci.yml` | Small Vite Vue TypeScript UI. |

## Local Checks

```bash
(cd fixtures/go-hello && go test ./...)
cargo test --manifest-path fixtures/rust-hello/Cargo.toml
```

Frontend fixtures need dependencies first:

```bash
(cd fixtures/react-hello && npm install && npm run build)
(cd fixtures/vue-hello && npm install && npm run build)
```

## Workflow Calls

Use these working directories when testing yoc from a caller workflow:

```yaml
with:
  working-directory: fixtures/go-hello
```

```yaml
with:
  working-directory: fixtures/rust-hello
```

For the frontend fixtures, override install and skip missing lint/test scripts:

```yaml
with:
  working-directory: fixtures/react-hello
  cache: false
  install-command: npm install
  lint-command: ""
  test-command: ""
```
