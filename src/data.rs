use chrono::prelude::*;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

// Store the created entries.
pub static DATA: Lazy<Mutex<Vec<Log>>> = Lazy::new(|| Mutex::new(vec![]));

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Log {
    customer_id: String,
    pub log_type: String,
    log_text: String,
    ts_rfc3339: String,
    namespace: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnstructuredLogs {
    customer_id: String,
    pub log_type: String,
    namespace: Option<String>,
    entries: Vec<UnstructuredLog>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UnstructuredLog {
    log_text: String,
    ts_epoch_microseconds: Option<i64>,
    ts_rfc3339: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UdmEvents {
    pub customer_id: String,
    pub events: Vec<UdmEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UdmEvent {
    metadata: UdmMetadata,
    pub invalid: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UdmMetadata {
    log_type: String,
    namespace: Option<String>,
    ts_epoch_microseconds: Option<i64>,
    ts_rfc3339: Option<String>,
}

impl From<UnstructuredLogs> for Vec<Log> {
    fn from(logs: UnstructuredLogs) -> Self {
        logs.entries
            .into_iter()
            .map(|entry| Log {
                customer_id: logs.customer_id.clone(),
                log_type: logs.log_type.clone(),
                log_text: entry.log_text,
                ts_rfc3339: entry.ts_rfc3339.unwrap_or_else(|| {
                    entry
                        .ts_epoch_microseconds
                        .map(|ms| Utc.timestamp_millis(ms))
                        .unwrap_or_else(Utc::now)
                        .to_rfc3339()
                }),
                namespace: logs.namespace.clone()
            })
            .collect()
    }
}

impl From<UdmEvents> for Vec<Log> {
    fn from(logs: UdmEvents) -> Self {
        logs.events.into_iter().map(|event| Log {
            customer_id: logs.customer_id.clone(),
            log_type: event.metadata.log_type.clone(),
            log_text: String::new(),
            namespace: event.metadata.namespace.clone(),
            ts_rfc3339: event.metadata.ts_rfc3339.unwrap_or_else(|| {
                event.metadata
                    .ts_epoch_microseconds
                    .map(|ms| Utc.timestamp_millis(ms))
                    .unwrap_or_else(Utc::now)
                    .to_rfc3339()
            }),
        }).collect()
    }
}

/// Adds the logs to our (in memory) database.
pub fn add_unstructured_to_data(logs: UnstructuredLogs) {
    let mut data = DATA.lock().unwrap();
    data.append(&mut Vec::from(logs));
}

/// Adds the logs to our (in memory) database.
pub fn add_udm_events_to_data(logs: UdmEvents) {
    let mut data = DATA.lock().unwrap();
    data.append(&mut Vec::from(logs));
}
