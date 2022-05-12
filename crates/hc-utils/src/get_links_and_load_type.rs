use crate::error::*;
use hdk::prelude::*;
use std::convert::TryFrom;

/// Gets the entries that are linked to a base with LinkTag by matching with the declared TryFrom Entry.
/// include_latest_updated_entry is used when an entry is updated in the zome
/// and if you need the latest update of those entries
pub fn get_links_and_load_type<R: TryFrom<Entry>>(
    base: EntryHash,
    tag: Option<LinkTag>,
    include_latest_updated_entry: bool,
) -> UtilsResult<Vec<R>> {
    let link_info = get_links(base, tag)?;
    if include_latest_updated_entry {
        let entries: Vec<Entry> = super::get_latest_entries(link_info, GetOptions::default())?;
        Ok(entries
            .iter()
            .flat_map(|entry| match R::try_from(entry.clone()) {
                Ok(e) => Ok(e),
                Err(_) => Err(UtilsError::Generic(
                    "Could not convert get_links result to requested type",
                )),
            })
            .collect())
    } else {
        let all_results_elements = super::get_details(link_info, GetOptions::default())?;
        Ok(all_results_elements
            .iter()
            .flat_map(|link| match link {
                Some(Details::Entry(EntryDetails { entry, .. })) => {
                    match R::try_from(entry.clone()) {
                        Ok(e) => Ok(e),
                        Err(_) => Err(UtilsError::Generic(
                            "Could not convert get_links result to requested type",
                        )),
                    }
                }
                _ => Err(UtilsError::Generic("get_links did not return an app entry")),
            })
            .collect())
    }
}

#[macro_export]
macro_rules! get_links_and_load_type {
    ($a: expr, $b: expr) => {
        get_links_and_load_type($a, $b, false)
    };
    ($a: expr, $b: expr, $c: expr) => {
        get_links_and_load_type($a, $b, $c)
    };
}
