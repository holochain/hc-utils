use crate::error::*;
use hdk::prelude::*;
use holochain_deterministic_integrity::prelude::link::LinkTypeRange;

pub fn remove_link(base: EntryHash, target: EntryHash, tag: LinkTag) -> UtilsResult<ActionHash> {
    let target = AnyLinkableHash::from(target);
    match get_links(base, LinkTypeRange::Full, Some(tag))?
        .into_iter()
        .find(|link| target == link.target)
    {
        Some(links) => return Ok(delete_link(links.create_link_hash)?),
        None => return Err(UtilsError::RemoveLinkError),
    }
}
