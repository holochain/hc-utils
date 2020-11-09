use crate::error::*;
use hdk3::prelude::*;
use link::Link;

pub fn get_latest_link(base: EntryHash, tag: LinkTag) -> UtilsResult<Option<Link>> {
    let profile_info = get_links!(base.into(), tag)?.into_inner();

    // Find the latest
    let latest_info =
        profile_info
            .into_iter()
            .fold(None, |latest: Option<Link>, link| match latest {
                Some(latest) => {
                    if link.timestamp > latest.timestamp {
                        Some(link)
                    } else {
                        Some(latest)
                    }
                }
                None => Some(link),
            });
    return Ok(latest_info);
}
