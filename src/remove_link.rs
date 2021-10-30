use hdk::prelude::*;
use crate::error::*;

pub fn remove_link(base: EntryHash, target: EntryHash, tag: LinkTag) -> UtilsResult<HeaderHash> {
    match get_links(base, Some(tag))?
        .into_inner()
        .into_iter()
        .find(|link| target == link.target)
    {
        Some(links) => return Ok(delete_link(links.create_link_hash)?),
        None => return Err(UtilsError::RemoveLinkError),
    }
}
