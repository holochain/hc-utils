use hdk::prelude::*;

/// Query for an existing Link in the local source-chain matching the given LinkType name(s).  
/// If one exists, return it Address, otherwise commit it.
pub fn create_idempotent_link<T, E>(
    base_address: impl Into<AnyLinkableHash>,
    target_address: impl Into<AnyLinkableHash>,
    link_type: T,
    tag: impl Into<LinkTag>,
) -> ExternResult<ActionHash>
where
    ScopedLinkType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    let list_of_links = query(ChainQueryFilter::new().action_type(ActionType::CreateLink))?;
    let base_address = base_address.into();
    let target_address = target_address.into();
    let tag = tag.into();
    for record in list_of_links {
        match record.action() {
            Action::CreateLink(c) => {
                if c.base_address == base_address
                    && c.target_address == target_address.clone()
                    && c.tag == tag.clone()
                {
                    return Ok(record.action_address().to_owned());
                }
            }
            _ => unreachable!(),
        };
    }
    let action = create_link(base_address, target_address, link_type, tag)?;
    Ok(action)
}
