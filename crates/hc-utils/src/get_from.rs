use hdk::prelude::*;

pub fn get_from<E: TryFrom<Entry>, H>(hash: H) -> ExternResult<(E, Record)>
where
    AnyDhtHash: From<H>,
{
    let reserve_action = get(hash, GetOptions::latest())?;
    if let Some(record) = reserve_action {
        if let RecordEntry::Present(entry) = record.entry() {
            let entry =
                E::try_from(entry.to_owned()).or(Err(wasm_error!("unable to serialize entry")))?;
            return Ok((entry, record));
        }
    }
    Err(wasm_error!("no entry found"))
}
