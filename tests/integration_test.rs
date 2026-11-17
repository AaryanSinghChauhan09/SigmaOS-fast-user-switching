//! Integration tests for SigmaOS-fast-user-switching.

use sigmaos_fast_user_switching_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
