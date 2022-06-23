use crate::error::*;
use hdk::prelude::*;

/// Query for an existing Entry in the local source-chain matching the given EntryType name(s).  If
/// one exists, return it Address, otherwise commit it.
pub fn commit_idempotent(value: Entry) -> UtilsResult<ActionHash> {
    for record in super::local_source_chain()? {
        if let record::RecordEntry::Present(entry) = record.entry() {
            if entry.clone() == value {
                return Ok(record.action_address().clone());
            }
        }
    }
    let result = create(CreateInput {
        entry_location: EntryDefLocation::app(0),
        entry_visibility: EntryVisibility::Public,
        entry: value,
        chain_top_ordering: ChainTopOrdering::Strict,
    })?;
    Ok(result)
}
