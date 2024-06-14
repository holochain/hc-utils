use hdk::prelude::*;

pub fn remove_link(input: GetLinksInput, target: EntryHash) -> ExternResult<ActionHash> {
    let target = AnyLinkableHash::from(target);
    match get_links(input)?
        .into_iter()
        .find(|link| target == link.target)
    {
        Some(links) => return Ok(delete_link(links.create_link_hash)?),
        None => {
            return Err(wasm_error!(WasmErrorInner::Guest(
                "Link does not exist".to_string()
            )))
        }
    }
}
