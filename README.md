![Rust](https://github.com/tschuett/aarch64_features/workflows/Rust/badge.svg) ![MSRV](https://img.shields.io/badge/msrv-1.73-red)

## aarch64_features - A feature detector for AArch64 features, i.e., FEAT_LSE?

This crate checks for available features of AArch64 cores. It strives
for completeness instead of focussing on the favorite features.

Update your Cargo.toml
 ```toml
 [dependencies]
 aarch64_features = "0.1.0"
 ```
and then
 ```rust
 use aarch64_features::{check_features, Feature};

 fn main() {
   let features = check_features();

   if features.contains(&Feature::FEAT_LSE) {
     println!("happy");
   }
}
 ```

supported configurations:

- Linux AArch64
- macOS AArch64
- Windows on ARM.
- anything not AArch64

## Caveats

The Linux kernel only exposes a subset of the features to userspace,
see [feature
registers](https://github.com/torvalds/linux/blob/master/Documentation/arm64/cpu-feature-registers.rst). The set
changes over time and thus the reported features of this crate can
change between versions.

If you have a big little cpu, then you may want to pin the current thread.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
