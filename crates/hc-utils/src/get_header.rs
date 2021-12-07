use crate::error::*;
use hdk::prelude::*;

pub fn get_header(entry: EntryHash) -> UtilsResult<HeaderHash> {
    match get(entry, Default::default())? {
        Some(element) => Ok(element.header_address().to_owned()),
        None => Err(UtilsError::EntryNotFound),
    }
}

#[macro_export]
macro_rules! get_header {
    ($a: expr) => {
        super::get_header::get_header($a)
    };
}
