# Working on omaTUNES

## Build and verification

- `Cargo.lock` is local and ignored. Refresh dependencies once at the start of a
  task with `rustup update beta && rustup run beta cargo update`; beta honors the
  configured registry minimum publish age. Build and test with stable Rust.
- Use Cargo's configured shared target directory; do not override it or assume
  binaries are under the checkout's `target/`. `cargo metadata --format-version 1
  --no-deps` reports `target_directory`.
- Run `cargo +stable test`. The library and binary currently declare the same
  modules, so their unit tests run twice. Existing compiler warnings are present.
- Validate launcher edits with `desktop-file-validate scripts/omatunes.desktop`
  and `bash -n scripts/omatunes-test.sh`.

## Isolated GUI checks

- Override `HOME` as well as XDG directories: core config/database paths use
  `$HOME/.config/omatunes`, not `XDG_CONFIG_HOME`. Use an empty music directory
  to avoid scanning or changing a real library.
- GPU logs show adapter selection, not successful display output. Verify the
  actual window is visible and foregrounded before judging screenshots; grouped
  Hyprland windows can occupy the same coordinates while another app is visible.
- `scripts/omatunes-test.sh` is the original author's NVIDIA-specific launcher
  with hard-coded paths. It is not a portable test runner.
