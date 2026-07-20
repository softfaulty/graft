# repository hygiene

## tracked files

- keep source, manifests, documentation and reproducibility metadata in git
- keep `Cargo.lock` because graft distributes applications
- keep gradle wrapper files when the android project is generated
- keep small test fixtures only with their source, license and expected hash documented
- normalize text to lf through `.gitattributes`; platform scripts declare exceptions there

## generated files

build output belongs in `target`, `build`, `out` or `dist` and stays untracked. generated source is tracked only when a required consumer cannot generate it and the producing command is documented beside it. do not hand-edit tracked generated output.

large images, recovery archives and device captures do not enter git by accident. sanitized fixtures belong under `testdata` with provenance and hashes. release artifacts are produced from a clean tree and distributed separately.

## secrets

credentials, signing keys, device backups, private state and local environment files never enter the repository. use `.env.example` for names and safe placeholder values. use local ignored paths for real material.

scan committed history and the current working tree before sharing changes:

```sh
gitleaks git --config .gitleaks.toml .
gitleaks dir --config .gitleaks.toml .
```

`.gitleaks.toml` extends the scanner's maintained default rules. a false positive needs the narrowest possible documented exception; never add a broad path allowlist to make the check green.

## pre-commit hook

enable the repository-owned hook once per clone:

```sh
git config core.hooksPath .githooks
```

the hook requires `cargo` and `gitleaks` on `PATH`. it scans staged changes, checks rust formatting and runs the workspace tests. run `.githooks/pre-commit` directly when debugging a failed hook.
