![Rust](https://github.com/tschuett/aarch64_features/workflows/Rust/badge.svg) ![MSRV](https://img.shields.io/badge/msrv-1.63-red)

# aarch64_features - A feature detector for AArch64 features, i.e, FEAT_LSE?

This crate checks for available features of AArch64 cores. It strives
for completeness instead of focussing on the favorite features.

supported configurations:

- Linux AArch64
- macOS AArch64
- anything not AArch64

# Caveats

The Linux kernel only exposes a subset of the features to userspace,
see [feature
registers](https://github.com/torvalds/linux/blob/master/Documentation/arm64/cpu-feature-registers.rst). The set
changes over time and thus the reported features of this crate can
change between versions.

Windows on ARM fails to compile.

If you have a big little cpu, then you may want to pin the current thread.
