use hdk::prelude::*;

/// optimized get by links
pub fn get(links: Vec<Link>, option: GetOptions) -> ExternResult<Vec<Option<Record>>> {
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
    Ok(HDK.with(|hdk| hdk.borrow().get(msg_results_input))?)
}
