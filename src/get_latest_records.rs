use hdk::prelude::*;

enum Latest {
    Found(Record),
    Continue(ActionHash),
    NoRecord,
}

pub fn get_latest_records(
    target: Vec<Link>,
    option: GetOptions,
) -> ExternResult<Vec<(Record, Link)>> {
    // Get the original
    let initial_details = super::get_details(target, option.clone())?;
    initial_details
        .into_iter()
        .map(|details| {
            let mut latest_details = check_updates(details.0)?;
            // Follow any updates until there are none
            // and choose the update with the latest timestamp
            loop {
                match latest_details {
                    // Found an entry with no more updates
                    Latest::Found(record) => return Ok((record, details.1)),
                    // Found an update so follow it
                    Latest::Continue(action_hash) => {
                        latest_details = _helper_get_latest_record(action_hash, option.clone())?
                    }
                    // There was no original so return the default
                    Latest::NoRecord => {
                        return Err(wasm_error!(WasmErrorInner::Guest(
                            "Unable to find record in dht".to_string()
                        )))
                    }
                }
            }
        })
        .collect()
}

fn _helper_get_latest_record(target: ActionHash, option: GetOptions) -> ExternResult<Latest> {
    let details = get_details(target, option.clone())?;
    match details {
        Some(details) => check_updates(details),
        None => Ok(Latest::NoRecord),
    }
}

// Get the actual profile entry
fn check_updates(details: Details) -> ExternResult<Latest> {
    match details {
        Details::Record(RecordDetails {
            record,
            updates,
            deletes,
            ..
        }) => {
            // No updates, we are done
            if deletes.len() > 0 {
                Ok(Latest::NoRecord)
            } else if updates.is_empty() {
                Ok(Latest::Found(record))
            } else {
                // Get the latest update via timestamp
                let latest_record = updates
                    .into_iter()
                    .fold(
                        None,
                        |latest: Option<SignedActionHashed>, update| match latest {
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
                Ok(Latest::Continue(latest_record.action_address().clone()))
            }
        }
        _ => Ok(Latest::NoRecord),
    }
}
