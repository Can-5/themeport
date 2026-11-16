---
name: Feature request
about: Suggest a new feature or improvement for ThemePort
title: "[Feature]: "
labels: [enhancement]
---

## Problem

What problem does this solve? Who is affected, and how often?

## Proposed solution

Describe what you would like to happen. If you have API or behavior details in
mind, include them.

## Alternatives considered

What other approaches did you consider, and why are they not sufficient?

## Does this change require network/OS access?

Answer honestly:

- [ ] No — this stays entirely on-device, like the rest of ThemePort.
- [ ] Yes — it would require network access or new OS-level permissions.

If the answer is "yes, data leaves the device by design", this needs an
explicit permission discussion in the issue **before** any implementation —
see [CONTRIBUTING.md](../CONTRIBUTING.md). ThemePort has no telemetry and no
network code today; that is a deliberate design constraint, not an oversight.
