use hdk::prelude::*;
use crate::error::*;

pub fn create_idempotent_link(base: EntryHash, target: EntryHash, link_tag: LinkTag)-> UtilsResult<HeaderHash> {
    let list_of_links = query(ChainQueryFilter::new().header_type(HeaderType::CreateLink))?;
    for element in list_of_links {
        match element.header() {
            Header::CreateLink(c) => {
                if c.base_address == base && c.target_address == target.clone().into() && c.tag == link_tag {
                    return Ok(element.header_address().to_owned())
                }
            }
            _ => unreachable!(),
        };
    };
    let header = create_link(base, target.into(), link_tag)?;
    Ok(header)
}
