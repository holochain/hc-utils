use hdk::prelude::*;

pub fn remove_link(
    query: LinkQuery,
    strategy: GetStrategy,
    target: AnyLinkableHash,
) -> ExternResult<ActionHash> {
    match get_links(query, strategy)?
        .into_iter()
        .find(|link| target == link.target)
    {
        Some(links) => {
            return Ok(delete_link(
                links.create_link_hash,
                GetOptions::new(strategy),
            )?)
        }
        None => {
            return Err(wasm_error!(WasmErrorInner::Guest(
                "Link does not exist".to_string()
            )))
        }
    }
}
