use crate::get_latest_records::LatestRecord;
use hdk::prelude::*;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoadedLinks<R> {
    pub original_record_hash: ActionHash,
    pub entry: R,
    pub record: Record,
    pub found_link: Link,
}

/// Gets the entries that are linked to a base with LinkTag by matching with the declared TryFrom Entry.
/// include_latest_updated_entry is used when an entry is updated in the zome
/// and if you need the latest update of those entries
pub fn get_links_and_load_type<R: TryFrom<Entry>>(
    input: GetLinksInput,
) -> ExternResult<Vec<LoadedLinks<R>>> {
    let link_info = get_links(input)?;
    let entries: Vec<LatestRecord> = super::get_latest_records(link_info, GetOptions::default())?;
    Ok(entries
        .iter()
        .flat_map(|latest_record| {
            let LatestRecord {
                original_record_hash,
                record,
                link,
            } = latest_record;
            if let RecordEntry::Present(entry) = &record.entry {
                match R::try_from(entry.clone()) {
                    Ok(e) => Ok(LoadedLinks {
                        original_record_hash: original_record_hash.clone(),
                        entry: e,
                        record: record.clone(),
                        found_link: link.clone(),
                    }),
                    Err(_) => Err(wasm_error!(
                        "Could not convert get_links result to requested type"
                    )),
                }
            } else {
                Err(wasm_error!("get_links did not return an app entry"))
            }
        })
        .collect())
}
