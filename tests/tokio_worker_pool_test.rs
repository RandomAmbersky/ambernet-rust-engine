extern crate asn_core_bus;
extern crate tokio_bus;

use asn_core_bus::AsnWorkerPool;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio_bus::new_worker_pool;

#[test]
fn test_run_main_thread() {
    let pool = new_worker_pool();
    let called = Arc::new(AtomicBool::new(false));
    let called_clone = called.clone();

    pool.run_main_thread(move || {
        called_clone.store(true, Ordering::SeqCst);
        Ok(())
    });

    assert!(
        called.load(Ordering::SeqCst),
        "Main thread function should be called"
    );
}
