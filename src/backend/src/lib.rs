#[macro_use]
extern crate serde;
mod store;
mod types;

use ic_cdk::api::time;
use store::*;
use types::*;

// ... (existing imports and types)

// get_health_record Function:
#[ic_cdk::query]
fn get_health_record(id: u64) -> Result<HealthRecord, Error> {
    match _get_health_record(&id) {
        Some(record) => Ok(record),
        None => Err(Error::NotFound {
            msg: format!("a health record with id={} not found", id),
        }),
    }
}

// 2.7.3 add_health_record Function:
#[ic_cdk::update]
fn add_health_record(record: PatientUpdatePayload) -> Option<HealthRecord> {
    let id = HEALTH_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for health records");
    let timestamp = time();
    let health_record = HealthRecord {
        id,
        patient_name: record.patient_name,
        age: record.age,
        symptoms: record.symptoms,
        diagnosis: record.diagnosis,
        medications: record.medications,
        monitoring_data: record.monitoring_data,
        medication_history: [].to_vec(),
        created_at: timestamp,
        updated_at: None,
    };
    do_insert_health_record(&health_record);
    Some(health_record)
}

// 2.7.4 update_health_record Function:
#[ic_cdk::update]
fn update_health_record(id: u64, payload: PatientUpdatePayload) -> Result<HealthRecord, Error> {
    match HEALTH_STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut health_record) => {
            health_record.patient_name = payload.patient_name;
            health_record.age = payload.age;
            health_record.symptoms = payload.symptoms;
            health_record.diagnosis = payload.diagnosis;
            health_record.medications = payload.medications;
            health_record.monitoring_data = payload.monitoring_data;
            health_record.updated_at = Some(time());
            do_insert_health_record(&health_record);
            Ok(health_record)
        }
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't update a health record with id={}. record not found",
                id
            ),
        }),
    }
}

// 2.7.5 delete_health_record Function:
#[ic_cdk::update]
fn delete_health_record(id: u64) -> Result<HealthRecord, Error> {
    match HEALTH_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(health_record) => Ok(health_record),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a health record with id={}. record not found.",
                id
            ),
        }),
    }
}
// function to get all health records
#[ic_cdk::query]
fn get_all_health_records() -> Vec<HealthRecord> {
    HEALTH_STORAGE.with(|service| {
        let borrow = service.borrow();
        borrow.iter().map(|(_, value)| value.clone()).collect()
    })
}

// New update to track medication for a specific health record
#[ic_cdk::update]
fn add_medication_record(id: u64, medication: String) -> Result<(), Error> {
    HEALTH_STORAGE.with(|service| {
        let mut storage = service.borrow_mut();

        // Check if the key exists
        if !storage.contains_key(&id) {
            // If not, insert a new HealthRecord
            let health_record = HealthRecord::default();
            storage.insert(id, health_record);
        }

        // Now, you can safely modify the HealthRecord
        if let Some(mut health_record) = storage.get(&id) {
            health_record.track_medication(medication);
            storage.insert(id, health_record);
        }

        Ok(())
    })
}

// New query to get medication history for a specific health record
#[ic_cdk::query]
fn get_medication_history(id: u64) -> Result<Vec<String>, Error> {
    match _get_health_record(&id) {
        Some(record) => Ok(record.medication_history.clone()),
        None => Err(Error::NotFound {
            msg: format!("a health record with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn get_total_health_records() -> u64 {
    HEALTH_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.len()
    })
}

#[ic_cdk::query]
fn get_latest_health_record_by_update_time(id: u64) -> Result<u64, Error> {
    match _get_health_record(&id) {
        Some(record) => Ok(record.updated_at.unwrap_or(record.created_at)),
        None => Err(Error::NotFound {
            msg: format!("a health record with id={} not found", id),
        }),
    }
}

#[ic_cdk::query]
fn health_record_exists(id: u64) -> bool {
    HEALTH_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.contains_key(&id)
    })
}

#[ic_cdk::query]
fn generate_health_report(id: u64) -> Result<String, Error> {
    match _get_health_record(&id) {
        Some(record) => {
            let report = format!(
                "Health Report for Patient {} (ID: {})\nAge: {}\nSymptoms: {}\nDiagnosis: {}\nMedications: {}\nMonitoring Data: {}",
                record.patient_name, record.id, record.age, record.symptoms, record.diagnosis, record.medications, record.monitoring_data
            );
            Ok(report)
        }
        None => Err(Error::GenerateFailed {
            msg: format!("a health record with id={} not found", id),
        }),
    }
}

// 2.7.7 enum Error:

// To generate the Candid interface definitions for our canister
ic_cdk::export_candid!();
