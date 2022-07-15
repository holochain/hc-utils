use crate::error::*;
use hdk::prelude::*;

/// Query for an existing Link in the local source-chain matching the given LinkType name(s).  
/// If one exists, return it Address, otherwise commit it.
pub fn create_idempotent_link(
    base: EntryHash,
    target: EntryHash,
    link_type: LinkType,
    link_tag: LinkTag,
) -> UtilsResult<ActionHash> {
    let list_of_links = query(ChainQueryFilter::new().action_type(ActionType::CreateLink))?;
    let base = base.into();
    for record in list_of_links {
        match record.action() {
            Action::CreateLink(c) => {
                if c.base_address == base
                    && c.target_address == target.clone().into()
                    && c.tag == link_tag
                {
                    return Ok(record.action_address().to_owned());
                }
            }
            _ => unreachable!(),
        };
    }
    let action = create_link(base, target, link_type, link_tag)?;
    Ok(action)
}
