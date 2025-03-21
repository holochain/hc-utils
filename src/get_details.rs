use hdk::prelude::*;

/// optimized get details by links
pub fn get_details(
    links: Vec<Link>,
    option: GetOptions,
) -> ExternResult<Vec<(Details, Link, ActionHash)>> {
    let links: Vec<(AnyDhtHash, Link)> = links
        .iter()
        .filter_map(|l| match l.clone().target.into_entry_hash() {
            Some(e) => Some((e.into(), l.clone())),
            None => match l.clone().target.into_action_hash() {
                Some(a) => Some((a.into(), l.clone())),
                None => None,
            },
        })
        .collect();
    let msg_results_input: Vec<GetInput> = links
        .clone()
        .into_iter()
        .map(|link| (GetInput::new(link.0, option.clone())))
        .collect();
    let list = HDK.with(|hdk| hdk.borrow().get_details(msg_results_input))?;

    Ok(list
        .iter()
        .filter_map(|d| match d {
            Some(details) => {
                if let Details::Record(r) = details {
                    let action_hash = r.record.action_address().clone();
                    let any_hash: AnyDhtHash = action_hash.clone().into();
                    let found = links
                        .iter()
                        .find_map(|(h, l)| if h == &any_hash { Some(l) } else { None });
                    match found {
                        Some(l) => Some((details.clone(), l.clone(), action_hash)),
                        None => None,
                    }
                } else {
                    //  Details::Entry(e): this is not needed for our app, but thats because we will alway link to ActionHash
                    None
                }
            }
            None => None,
        })
        .collect())
}
