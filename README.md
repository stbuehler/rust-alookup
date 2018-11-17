[![Travis Build Status](https://travis-ci.org/stbuehler/rust-alookup.svg?branch=master)](https://travis-ci.org/stbuehler/rust-alookup)
[![crates.io](https://img.shields.io/crates/v/alookup.svg)](https://crates.io/crates/alookup)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

Lookup IPv4 and IPv6 addresses for a hostname. Only prints addresses on
`stdout` (one per line), errors to `stderr`, and hard errors can be
detected through inspecting the exit code.

Install from [`crates.io`](https://crates.io/crates/alookup) with
`cargo install alookup`.

Exit codes:

- `0`: success (or `NODATA`).  You might want to treat an empty address
  set (no output) as failure too (similar to `NXDOMAIN`).
- `1`: name not found (`NXDOMAIN`).  If an empty address set is ok for
  you, you might want to ignore this exit code.
- `2`: `SRVFAIL`, timeouts, failed parsing response, generic resolver failure...
- `3`: failed parsing a specific answer record (might have printed
  partial result, but breaks on first broken record)

Other exit codes should be treated as failures too; a non-zero exit code
always should show an error on `stderr`, and every time an error is
printed to `stderr` there should be a non-zero exit code.
