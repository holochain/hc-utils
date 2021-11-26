use crate::error::*;
use hdk::prelude::*;

#[deprecated(note = "Switch to using the macro remove_link!() instead")]
pub fn remove_link(base: EntryHash, target: EntryHash, tag: LinkTag) -> UtilsResult<HeaderHash> {
    match get_links(base, Some(tag))?
        .into_iter()
        .find(|link| target == link.target)
    {
        Some(links) => return Ok(delete_link(links.create_link_hash)?),
        None => return Err(UtilsError::RemoveLinkError),
    }
}

#[macro_export]
macro_rules! remove_link {
    ($a: expr, $b: expr) => {
        super::remove_link::remove_link($a, $b)
    };
}
