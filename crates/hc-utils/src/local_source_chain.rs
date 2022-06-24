use crate::error::*;
use hdk::prelude::*;

/// Returns a list of records from the local source-chain
pub fn local_source_chain() -> UtilsResult<Vec<Record>> {
    let filter = QueryFilter::new();
    let with_entry_filter = filter.include_entries(true);

    // TODO: How to we filter for a perticular entry
    // let entry_filter = with_entry_filter.entry_type(EntryType::App(AppEntryType::new(
    //     EntryDefIndex::from(0),
    //     ZomeId::from(0),
    //     EntryVisibility::Public,
    // )));

    let action_filter = with_entry_filter.action_type(ActionType::Create);
    let query_result: Vec<Record> = query(action_filter)?;
    Ok(query_result)
}
