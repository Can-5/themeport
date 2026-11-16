# Contributing to ThemePort

Thanks for your interest in ThemePort! ThemePort is a standalone domain-engine crate
in the [Orbit ecosystem](https://github.com/Can-5/orbit): MIT-licensed, usable on
its own without Orbit and without the other module crates, and focused on
privacy-first local tooling.

It lives at [Can-5/themeport](https://github.com/Can-5/themeport).

## Build & test

```sh
cargo test --locked
cargo build --locked --release
```

Both must pass before you open a pull request. The crate's dependencies come
from crates.io only — no cross-repo path dependencies, no git dependencies.

## Ground rules

- **This crate must remain standalone — no dependencies on other Orbit repos.**
  If you think that needs to change, open an issue to discuss it first.
- **Small, focused PRs.** One concern per pull request. Avoid drive-by
  refactors mixed with behavior changes.
- **No telemetry. Ever.** No analytics, no crash pings, no "anonymous usage
  statistics", no background beacons. Any feature that would move data off the
  device requires an explicit, public permission discussion in an issue *before*
  implementation — and this crate has no network code at all.
- **No network I/O.** Do not add network access, sockets, HTTP clients, or
  download-at-build-time tricks. The build must not fetch anything beyond the
  initial crates.io registry resolution.
- **No fake features or fake system data.** If something is not implemented, it
  must be reported as unimplemented or unavailable — never simulated, stubbed
  with made-up values, or presented as working.
- **Tests are required.** Behavior changes must include tests that exercise the
  new behavior (`cargo test --locked`). Bug fixes should include a regression
  test where practical.
- **MIT license.** By contributing, you agree that your contributions are
  licensed under the [MIT License](LICENSE), the same license as the project.
- **English code.** Source code, comments, documentation, and commit messages
  are written in English.
- **Commit messages.** Conventional-Commit-ish style is appreciated:
  `feat: …`, `fix: …`, `docs: …`, `refactor: …`, `test: …`, `chore: …`.

## Issues and security

- Use the issue templates for bugs and feature requests.
- For vulnerabilities, do **not** open a public issue — follow
  [SECURITY.md](SECURITY.md) instead.
