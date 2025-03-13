pub mod io;

pub mod adaptor;
pub mod process;
mod payload;

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use self::adaptor::{Adaptor, EventStore};

static EXISTS: AtomicBool = AtomicBool::new(false);
static GLOBAL_INIT: AtomicUsize = AtomicUsize::new(UNINITIALIZED);
static mut EVENTSTORE: EventStore = EventStore { store: None };

const UNINITIALIZED: usize = 0;
const INITIALIZING: usize = 1;
const INITIALIZED: usize = 2;


pub fn setup<T: Adaptor>(install: T) {
    if GLOBAL_INIT
        .compare_exchange(
            UNINITIALIZED,
            INITIALIZING,
            Ordering::SeqCst,
            Ordering::SeqCst,
        )
        .is_ok()
    {
        unsafe {
            EVENTSTORE = EventStore::from(install);
        }
        GLOBAL_INIT.store(INITIALIZED, Ordering::SeqCst);
        EXISTS.store(true, Ordering::Release);
    } else {
        panic!("EventStore already initialized");
    }
}

pub fn get_eventstore() -> &'static EventStore {
    if GLOBAL_INIT.load(Ordering::SeqCst) != INITIALIZED {
        panic!("EventStore not initialized");
    }
    unsafe {
        &*std::ptr::addr_of!(EVENTSTORE)
    }
}