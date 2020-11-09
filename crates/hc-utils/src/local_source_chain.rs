use crate::error::*;
use hdk3::prelude::*;

pub fn local_source_chain() -> UtilsResult<ElementVec> {
    debug!("Getting local chain.")?;

    let filter = QueryFilter::new();
    let with_entry_filter = filter.include_entries(true);

    // TODO: How to we filter for a perticular entry
    // let entry_filter = with_entry_filter.entry_type(EntryType::App(AppEntryType::new(
    //     EntryDefIndex::from(0),
    //     ZomeId::from(0),
    //     EntryVisibility::Public,
    // )));

    let header_filter = with_entry_filter.header_type(HeaderType::Create);
    let query_result: ElementVec = query!(header_filter)?;
    debug!("returning local source chain.")?;
    Ok(query_result)
}
