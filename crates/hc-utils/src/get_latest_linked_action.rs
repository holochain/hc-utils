use hdk::prelude::*;

pub fn get_latest_linked_action(
    base: impl Into<AnyLinkableHash>,
    link_type: impl LinkTypeFilterExt,
    link_tag: Option<LinkTag>,
) -> ExternResult<Vec<Record>> {
    let links = get_links(base, link_type, link_tag)?;
    let records: Vec<Record> = links
        .into_iter()
        .filter_map(
            |link| match helper_get_latest_action(ActionHash::from(link.target).into()) {
                Ok(a) => a,
                _ => None,
            },
        )
        .collect();

    Ok(records)
}

fn helper_get_latest_action(action_hash: ActionHash) -> ExternResult<Option<Record>> {
    let details = get_details(action_hash, GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("Action not found".into())
    ))?;
    let record_details = match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(record_details),
    }?;
    if record_details.deletes.len() > 0 {
        return Ok(None);
    }
    match record_details.updates.last() {
        Some(update) => helper_get_latest_action(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
}
