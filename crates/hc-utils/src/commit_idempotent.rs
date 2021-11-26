use crate::error::*;
use hdk::prelude::*;

/// Query for an existing Entry in the local source-chain matching the given EntryType name(s).  If
/// one exists, return it Address, otherwise commit it.
#[deprecated(note = "Switch to using the macro commit_idempotent!() instead")]
pub fn commit_idempotent(entry_id: String, value: Entry) -> UtilsResult<HeaderHash> {
    for element in super::local_source_chain!()? {
        if let element::ElementEntry::Present(entry) = element.entry() {
            if entry.clone() == value {
                return Ok(element.header_address().clone());
            }
        }
    }
    let result = create(CreateInput {
        entry_def_id: EntryDefId::App(entry_id),
        entry: value,
        chain_top_ordering: ChainTopOrdering::Strict,
    })?;
    Ok(result)
}

#[macro_export]
macro_rules! commit_idempotent {
    ($a: expr, $b: expr) => {
        super::commit_idempotent::commit_idempotent($a, $b)
    };
}
