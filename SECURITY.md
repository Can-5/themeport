# Security Policy

## Supported versions

The latest minor release series is supported (currently 0.1.x). Older releases
do not receive security fixes — please upgrade to the latest release.

## Reporting a vulnerability

Please report vulnerabilities **privately** via GitHub's
["Report a vulnerability"](https://github.com/Can-5/themeport/security/advisories/new)
flow for this repository (Security tab → Report a vulnerability). This opens a
private advisory visible only to you and the maintainers.

- Do **not** open a public issue for a security report.
- There is no email contact for security reports and no bug bounty program —
  the private GitHub advisory is the only channel.
- There is no SLA; reports are handled on a best-effort basis by a small
  maintainer team.

### What to include

- A description of the vulnerability and its impact
- Steps to reproduce, ideally a minimal reproducer (code snippet, test case,
  or project state)
- The affected version or commit hash, and your platform/toolchain

## Coordinated disclosure

We follow coordinated disclosure: after a report is received and confirmed, we
will work on a fix, and the advisory will be published once a fix is available
(or the report is determined to be out of scope). You will be credited in the
advisory unless you ask otherwise. Please give us a reasonable window to ship a
fix before any public disclosure.

## No telemetry, no network

ThemePort has **no telemetry by design**: no analytics, no crash reporting, no
usage statistics, and no network code. Nothing leaves your device. If you
believe you have found behavior that violates this, that is a security issue —
please report it through the private advisory flow above.
