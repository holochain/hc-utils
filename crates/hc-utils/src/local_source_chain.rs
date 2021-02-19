use crate::error::*;
use hdk::prelude::*;

pub fn local_source_chain() -> UtilsResult<Vec<Element>> {
    let filter = QueryFilter::new();
    let with_entry_filter = filter.include_entries(true);

    // TODO: How to we filter for a perticular entry
    // let entry_filter = with_entry_filter.entry_type(EntryType::App(AppEntryType::new(
    //     EntryDefIndex::from(0),
    //     ZomeId::from(0),
    //     EntryVisibility::Public,
    // )));

    let header_filter = with_entry_filter.header_type(HeaderType::Create);
    let query_result: Vec<Element> = query(header_filter)?;
    Ok(query_result)
}
