use hdk::prelude::*;
use link::Link;

/// optimized get details by links
pub fn get_details(links: Vec<Link>, option: GetOptions) -> ExternResult<Vec<Option<Details>>> {
    let links: Vec<AnyDhtHash> = links
        .iter()
        .filter_map(|l| match l.clone().target.into_entry_hash() {
            Some(e) => Some(e.into()),
            None => match l.clone().target.into_action_hash() {
                Some(a) => Some(a.into()),
                None => None,
            },
        })
        .collect();

    let msg_results_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(link, option.clone()))
        .collect();
    Ok(HDK.with(|hdk| hdk.borrow().get_details(msg_results_input))?)
}
