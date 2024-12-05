## nexus examples

```shell
cd /src/guest
#  generate the zk proof
RUST_LOG=DEBUG cargo nexus prove

#  verify the zk proof
RUST_LOG=DEBUG cargo nexus verify
```

only run the main function
```shell
RUST_LOG=DEBUG cargo run --release
```

runtime log
```shell
warning: Patch `ark-bls12-381 v0.4.0 (https://github.com/arkworks-rs/curves/?rev=3fded1f#3fded1fb)` was not used in the crate graph.
Patch `ark-bn254 v0.4.0 (https://github.com/arkworks-rs/curves/?rev=8c0256a#8c0256ac)` was not used in the crate graph.
Patch `ark-crypto-primitives v0.4.0 (https://github.com/arkworks-rs/crypto-primitives/?rev=d27a5c8#d27a5c83)` was not used in the crate graph.
Patch `ark-ec v0.4.2 (https://github.com/arkworks-rs/algebra/?rev=2a80c54#2a80c546)` was not used in the crate graph.
Patch `ark-ff v0.4.2 (https://github.com/arkworks-rs/algebra/?rev=2a80c54#2a80c546)` was not used in the crate graph.
Patch `ark-pallas v0.4.0 (https://github.com/arkworks-rs/curves/?rev=8c0256a#8c0256ac)` was not used in the crate graph.
Patch `ark-poly v0.4.2 (https://github.com/arkworks-rs/algebra/?rev=2a80c54#2a80c546)` was not used in the crate graph.
Patch `ark-poly-commit v0.4.0 (https://github.com/arkworks-rs/poly-commit/?rev=12f5529#12f5529c)` was not used in the crate graph.
Patch `ark-r1cs-std v0.4.0 (https://github.com/arkworks-rs/r1cs-std/?rev=2ca3bd7#2ca3bd7d)` was not used in the crate graph.
Patch `ark-serialize v0.4.2 (https://github.com/arkworks-rs/algebra/?rev=2a80c54#2a80c546)` was not used in the crate graph.
Patch `ark-test-curves v0.4.2 (https://github.com/arkworks-rs/algebra/?rev=2a80c54#2a80c546)` was not used in the crate graph.
Patch `ark-vesta v0.4.0 (https://github.com/arkworks-rs/curves/?rev=8c0256a#8c0256ac)` was not used in the crate graph.
Check that the patched package version and available features are compatible
with the dependency requirements. If the patch has a different version from
what is locked in the Cargo.lock file, run `cargo update` to use the new
version. This may also occur with an optional dependency that is not enabled.
    Finished `release` profile [optimized] target(s) in 0.12s
  2024-12-05T11:34:19.764859+08:00  INFO nexus-cli: path ~/nexus-zkvm-guest-example/target/nexus-cache/nexus-public-nova-seq-16.zst already exists, use `setup --force` to overwrite

No private input provided...
  Loading public parameters .   0ms
  2024-12-05T11:34:26.684433+08:00  INFO nexus-prover: Loading public parameters, path: "~/nexus-zkvm-guest-example/target/nexus-cache/nexus-public-nova-seq-16.zst"
  Loading public parameters ... 4.2s
 Finished in 4.2s
  Computing step 0 ... 966ms
  Computing step 1 ... 1.2s
  Computing step 2 ... 1.1s
  Computing step 3 ... 1.2s
  Computing step 4 ... 1.1s
  Computing step 5 ... 1.1s
  Computing step 6 ... 1.2s
  Computing step 7 ... 1.2s
  Computing step 8 ... 1.2s
  Computing step 9 ... 1.2s
  Computing step 10 ... 1.1s
  Computing step 11 ... 1.2s
  Computing step 12 ... 1.2s
  Computing step 13 ... 1.2s
  Saving proof .   0ms
  2024-12-05T11:34:47.816032+08:00  INFO nexus-prover: Saving the proof, path: ~/nexus-zkvm-guest-example/src/guest/nexus-proof
  Saving proof ... 22ms
Finished in 22mss
```