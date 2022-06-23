use crate::error::*;
use hdk::prelude::*;

/// Query for an existing Entry in the local source-chain matching the given EntryType name(s).  If
/// one exists, return it Address, otherwise returns error.
pub fn exists(value: Entry) -> UtilsResult<ActionHash> {
    for record in super::local_source_chain().unwrap() {
        if let record::RecordEntry::Present(entry) = record.entry() {
            if entry.clone() == value {
                return Ok(record.action_address().clone());
            }
        }
    }
    return Err(UtilsError::EntryNotFound);
}
