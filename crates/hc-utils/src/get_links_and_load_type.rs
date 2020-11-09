use crate::error::*;
use crate::get_latest_entry::get_latest_entry;
use hdk3::prelude::*;
use std::convert::TryFrom;

pub fn get_links_and_load_type<R: TryFrom<Entry>>(
    base: EntryHash,
    tag: LinkTag,
) -> UtilsResult<Vec<R>> {
    let link_info = get_links!(base.into(), tag)?.into_inner();

    Ok(link_info
        .iter()
        .map(|link| match get_latest_entry(link.target.clone()) {
            Ok(entry) => match R::try_from(entry.clone()) {
                Ok(e) => Ok(e),
                Err(_) => Err(UtilsError::Generic(
                    "Could not convert get_links result to requested type",
                )),
            },
            _ => Err(UtilsError::Generic("get_links did not return an app entry")),
        })
        .filter_map(Result::ok)
        .collect())
}
