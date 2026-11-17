//! Integration tests for SigmaOS-systemd-coredump.

use sigmaos_systemd_coredump_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
