use crate::error::*;
use hdk::prelude::*;
use link::Link;

/// gets latest link created to the specific base
pub fn get_details(links: Vec<Link>, option: GetOptions) -> UtilsResult<Vec<Option<Details>>> {
    let msg_results_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(link.target.into(), option.clone()))
        .collect();
    Ok(HDK.with(|hdk| hdk.borrow().get_details(msg_results_input))?)
}

#[macro_export]
macro_rules! get_details {
    ($a: expr, $b: expr) => {
        super::get_details::get_details($a, $b)
    };
}
