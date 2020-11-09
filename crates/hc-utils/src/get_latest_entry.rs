use hdk3::prelude::*;
use metadata::EntryDetails;
enum Latest {
    Found(Entry),
    Continue(EntryHash),
    NoEntry,
}
use crate::error::*;

pub fn get_latest_entry(target: EntryHash) -> UtilsResult<Entry> {
    // Get the original
    let mut latest_profile = _get_latest_entry(target)?;

    // Follow any updates until there are none
    // and choose the update with the latest timestamp
    loop {
        match latest_profile {
            // Found an entry with no more updates
            Latest::Found(entry) => return Ok(entry),
            // Found an update so follow it
            Latest::Continue(entry_hash) => latest_profile = _get_latest_entry(entry_hash)?,
            // There was no original so return the default
            Latest::NoEntry => return Err(UtilsError::EntryNotFound),
        }
    }
}

// Get the actual profile entry
fn _get_latest_entry(entry: EntryHash) -> UtilsResult<Latest> {
    match get_details!(entry)? {
        Some(Details::Entry(EntryDetails { entry, updates, .. })) => {
            // No updates, we are done
            if updates.is_empty() {
                Ok(Latest::Found(entry))
            } else {
                // Get the latest update via timestamp
                let latest_entry = updates
                    .into_iter()
                    .fold(
                        None,
                        |latest: Option<element::SignedHeaderHashed>, update| match latest {
                            Some(latest) => {
                                if update.header().timestamp() > latest.header().timestamp() {
                                    Some(update)
                                } else {
                                    Some(latest)
                                }
                            }
                            None => Some(update),
                        },
                    )
                    .expect("Updates are not empty");
                match latest_entry.header().entry_hash() {
                    Some(header) => Ok(Latest::Continue(header.clone())),
                    None => unreachable!(),
                }
            }
        }
        _ => Ok(Latest::NoEntry),
    }
}
