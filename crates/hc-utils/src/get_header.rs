use hdk3::prelude::*;
pub fn get_header(entry: EntryHash) -> Result<HeaderHash, String> {
    match get!(entry)? {
        Some(element) => Ok(element.header_address().to_owned()),
        None => Err("Error: utils.get_header - EntryNotFound".to_string()),
    }
}
