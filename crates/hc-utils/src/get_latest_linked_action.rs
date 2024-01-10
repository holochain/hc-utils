use hdk::prelude::*;

pub fn get_latest_linked_action(input: GetLinksInput) -> ExternResult<Vec<Record>> {
    let links = get_links(input)?;

    let records: Vec<Record> = links
        .into_iter()
        .filter_map(
            |link| match helper_get_latest_action(link.target.into_any_dht_hash()) {
                Ok(a) => a,
                _ => None,
            },
        )
        .collect();

    Ok(records)
}

fn helper_get_latest_action<H: Into<AnyDhtHash>>(
    action_hash: Option<H>,
) -> ExternResult<Option<Record>> {
    match action_hash {
        Some(hash) => {
            let details = get_details(hash, GetOptions::default())?.ok_or(wasm_error!(
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
                Some(update) => helper_get_latest_action(Some(update.action_address().clone())),
                None => Ok(Some(record_details.record)),
            }
        }
        None => Ok(None),
    }
}
