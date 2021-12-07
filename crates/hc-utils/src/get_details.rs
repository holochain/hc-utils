use crate::error::*;
use hdk::prelude::*;
use link::Link;

/// optimized get details by links
pub fn get_details(links: Vec<Link>, option: GetOptions) -> UtilsResult<Vec<Option<Details>>> {
    let msg_results_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), option.clone()))
        .collect();
    Ok(HDK.with(|hdk| hdk.borrow().get_details(msg_results_input))?)
}
