use hdk::prelude::*;

/// Query for an existing Entry in the local source-chain matching the given EntryType name(s).  If
/// one exists, return it Address, otherwise commit it.
pub fn commit_idempotent(value: CreateInput) -> ExternResult<ActionHash> {
    for record in super::local_source_chain()? {
        if let RecordEntry::Present(entry) = record.entry() {
            if entry.clone() == value.entry {
                return Ok(record.action_address().clone());
            }
        }
    }
    let result = create(value)?;
    Ok(result)
}
