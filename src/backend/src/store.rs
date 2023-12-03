use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{Cell, DefaultMemoryImpl, StableBTreeMap};
use std::cell::RefCell;

use crate::types::HealthRecord;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;


// New thread-local variables for our Virtual Healthcare Assistant (VHA) app

thread_local! {
    static HEALTH_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

   pub static HEALTH_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(HEALTH_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for health records")
    );

   pub static HEALTH_STORAGE: RefCell<StableBTreeMap<u64, HealthRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            HEALTH_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for HealthRecord
pub fn do_insert_health_record(record: &HealthRecord) {
    HEALTH_STORAGE.with(|service| service.borrow_mut().insert(record.id, record.clone()));
}

// 2.7.2 _get_health_record Function:
pub fn _get_health_record(id: &u64) -> Option<HealthRecord> {
    HEALTH_STORAGE.with(|s| s.borrow().get(id))
}

