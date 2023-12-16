use std::borrow::Cow;

use candid::{Encode, Decode};
use ic_stable_structures::{Storable, BoundedStorable};

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct HealthRecord {
   pub id: u64,
   pub doctor_principal: String,
   pub patient_name: String,
   pub age: u32,
   pub symptoms: String,
   pub diagnosis: String,
   pub medications: String,
   pub medication_history: Vec<String>,
   pub monitoring_data: String,
   pub created_at: u64,
   pub updated_at: Option<u64>,
}

// Implementing Storable and BoundedStorable traits for HealthRecord
impl Storable for HealthRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for HealthRecord {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl HealthRecord {
    // ... (existing methods)

    // New method to track medication
   pub fn track_medication(&mut self, medication: String) {
        // Add the new medication to the medication history
        self.medication_history.push(medication);
    }
}


#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct PatientUpdatePayload {
   pub patient_name: String,
   pub age: u32,
   pub symptoms: String,
   pub diagnosis: String,
   pub medications: String,
   pub monitoring_data: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
    NotDoctor,
    GenerateFailed { msg: String },
    InputValidationFailed {msg: String}
}
