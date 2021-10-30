use crate::error::*;
use crate::local_source_chain::local_source_chain;
use hdk::prelude::*;

/// Query for an existing Entry in the local source-chain matching the given EntryType name(s).  If
/// one exists, return it Address, otherwise returns error.
pub fn exists(value: Entry) -> UtilsResult<HeaderHash> {
    for element in local_source_chain().unwrap() {
        if let element::ElementEntry::Present(entry) = element.entry() {
            if entry.clone() == value {
                return Ok(element.header_address().clone());
            }
        }
    }
    return Err(UtilsError::EntryNotFound);
}
