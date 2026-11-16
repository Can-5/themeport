# ThemePort

**Theme engine for Orbit: profiles, WCAG contrast, portable validated theme packs.**

> Orbit companion module — phase 2/4 of the [Orbit ecosystem](https://github.com/Can-5/orbit).
> Standalone crate: usable without Orbit, without other modules, and without network.

## Implemented now

- ThemeColor with luminance + contrast_ratio; meets_aa/meets_aaa; ThemeProfile with contrast_warnings()
- PackManifest: strict manifest (deny_unknown_fields — executable fields are rejected), slug/semver/unique-id validation, JSON roundtrip

## Not implemented yet (explicitly out of scope for this crate)

- No wallpaper application or icon switching — host layer, needs wallpaper write access
- No network fetch of themes — packs are local files/URLs the user supplies

## Build & test

```sh
cargo test  # 11 tests
```

Dependencies: serde, thiserror  # serde_json (dev). No cross-repo path dependencies, no Git dependencies, no downloads at build time beyond crates.io.

## Privacy

Themes are local files. No scripts run from packs — manifests with unknown or executable fields fail to parse. No network code.

## License

MIT © The Orbit contributors. See [LICENSE](./LICENSE).
