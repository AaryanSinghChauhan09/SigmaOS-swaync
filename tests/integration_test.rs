// Integration test for SigmaOS-swaync
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use sigmaos_swaync as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
