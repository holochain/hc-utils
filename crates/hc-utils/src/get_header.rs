use crate::error::*;
use hdk3::prelude::*;

pub fn get_header(entry: EntryHash) -> UtilsResult<HeaderHash> {
    match get!(entry)? {
        Some(element) => Ok(element.header_address().to_owned()),
        None => Err(UtilsError::EntryNotFound),
    }
}
