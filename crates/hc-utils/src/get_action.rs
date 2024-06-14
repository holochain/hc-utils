use hdk::prelude::*;

pub fn get_action(entry: EntryHash) -> ExternResult<ActionHash> {
    match get(entry, Default::default())? {
        Some(record) => Ok(record.action_address().to_owned()),
        None => Err(wasm_error!(WasmErrorInner::Guest(
            "Unable to find entry in dht".to_string()
        ))),
    }
}
