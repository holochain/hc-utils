use crate::error::*;
use hdk::prelude::*;

/// Query for an existing Link in the local source-chain matching the given LinkType name(s).  
/// If one exists, return it Address, otherwise commit it.
pub fn create_idempotent_link(
    base: EntryHash,
    target: EntryHash,
    link_type: LinkType,
    link_tag: LinkTag,
) -> UtilsResult<HeaderHash> {
    let list_of_links = query(ChainQueryFilter::new().header_type(HeaderType::CreateLink))?;
    for element in list_of_links {
        match element.header() {
            Header::CreateLink(c) => {
                if c.base_address == base
                    && c.target_address == target.clone().into()
                    && c.tag == link_tag
                {
                    return Ok(element.header_address().to_owned());
                }
            }
            _ => unreachable!(),
        };
    }
    let header = create_link(base, target.into(), link_type, link_tag)?;
    Ok(header)
}
