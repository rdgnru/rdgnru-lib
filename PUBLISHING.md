# Publishing `rdgnru-lib` to crates.io

## One-time setup

1. Create or sign in to your crates.io account.
2. Verify your email address in crates.io account settings.
3. Create a crates.io API token.
4. Log in locally:

```bash
cargo login
```

Paste the token when prompted. Keep the token secret.

## Before publishing

From the project root:

```bash
cargo test
cargo publish --dry-run
cargo package --list
```

Check that the generated package contains only the intended files.

Crate names are first-come-first-served on crates.io, so confirm that `rdgnru-lib` is available before relying on this name.

## Publish

```bash
cargo publish
```

After publishing, this exact version cannot be overwritten. To release updates, edit the `version` in `Cargo.toml`, update `CHANGELOG.md`, tag the release in git, and run `cargo publish` again.
