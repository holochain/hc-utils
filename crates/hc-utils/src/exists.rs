use crate::error::*;
use hdk::prelude::*;

/// Query for an existing Entry in the local source-chain matching the given EntryType name(s).  If
/// one exists, return it Address, otherwise returns error.
#[deprecated(note = "Switch to using the macro exists!() instead")]
pub fn exists(value: Entry) -> UtilsResult<HeaderHash> {
    for element in super::local_source_chain!().unwrap() {
        if let element::ElementEntry::Present(entry) = element.entry() {
            if entry.clone() == value {
                return Ok(element.header_address().clone());
            }
        }
    }
    return Err(UtilsError::EntryNotFound);
}

#[macro_export]
macro_rules! exists {
    ($a: expr) => {
        super::exists::exists($a)
    };
}
