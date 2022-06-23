use super::error::*;
use hdk::prelude::*;

enum Latest {
    Found(Entry),
    Continue(EntryHash),
    NoEntry,
}

/// Obtains the updates for the target Entry, and examines all of them to selects the latest one by
/// looking at the update time in its header.
///
/// An identical Entry can be committed by multiple Agents; this obtains the Entry's Header from the
/// perspective of *this* Agent.  It also may be committed by the same Agent multiple times, this
/// algorithm depends on either making the Entry unique, *or* that the caller is OK with it
/// returning the latest Update by any of this Agent's commits of this identical Entry.
pub fn get_latest_entry(target: EntryHash, option: GetOptions) -> UtilsResult<Entry> {
    // Get the original
    let mut latest_details = _helper_get_latest_entry(target, option.clone())?;

    // Follow any updates until there are none
    // and choose the update with the latest timestamp
    loop {
        match latest_details {
            // Found an entry with no more updates
            Latest::Found(entry) => return Ok(entry),
            // Found an update so follow it
            Latest::Continue(entry_hash) => {
                latest_details = _helper_get_latest_entry(entry_hash, option.clone())?
            }
            // There was no original so return the default
            Latest::NoEntry => return Err(UtilsError::EntryNotFound),
        }
    }
}

pub fn get_latest_entries(target: Vec<Link>, option: GetOptions) -> UtilsResult<Vec<Entry>> {
    // Get the original
    let initial_details = super::get_details(target, option.clone())?;
    initial_details
        .into_iter()
        .map(|details| {
            let mut latest_details = check_updates(details)?;
            // Follow any updates until there are none
            // and choose the update with the latest timestamp
            loop {
                match latest_details {
                    // Found an entry with no more updates
                    Latest::Found(entry) => return Ok(entry),
                    // Found an update so follow it
                    Latest::Continue(entry_hash) => {
                        latest_details = _helper_get_latest_entry(entry_hash, option.clone())?
                    }
                    // There was no original so return the default
                    Latest::NoEntry => return Err(UtilsError::EntryNotFound),
                }
            }
        })
        .collect()
}

fn _helper_get_latest_entry(target: EntryHash, option: GetOptions) -> UtilsResult<Latest> {
    let details = get_details(target, option.clone())?;
    check_updates(details)
}

// Get the actual profile entry
fn check_updates(details: Option<Details>) -> UtilsResult<Latest> {
    match details {
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
                        |latest: Option<record::SignedActionHashed>, update| match latest {
                            Some(latest) => {
                                if update.action().timestamp() > latest.action().timestamp() {
                                    Some(update)
                                } else {
                                    Some(latest)
                                }
                            }
                            None => Some(update),
                        },
                    )
                    .expect("Updates are not empty");
                match latest_entry.action().entry_hash() {
                    Some(header) => Ok(Latest::Continue(header.clone())),
                    None => unreachable!(),
                }
            }
        }
        _ => Ok(Latest::NoEntry),
    }
}
