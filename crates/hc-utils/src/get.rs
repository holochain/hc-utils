use crate::error::*;
use hdk::prelude::*;
use link::Link;

/// optimized get by links
pub fn get(links: Vec<Link>, option: GetOptions) -> UtilsResult<Vec<Option<Element>>> {
    let msg_results_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), option.clone()))
        .collect();
    Ok(HDK.with(|hdk| hdk.borrow().get(msg_results_input))?)
}
