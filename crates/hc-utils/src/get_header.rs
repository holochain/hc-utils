use crate::error::*;
use hdk::prelude::*;

pub fn get_header(entry: EntryHash) -> UtilsResult<ActionHash> {
    match get(entry, Default::default())? {
        Some(record) => Ok(record.action_address().to_owned()),
        None => Err(UtilsError::EntryNotFound),
    }
}
