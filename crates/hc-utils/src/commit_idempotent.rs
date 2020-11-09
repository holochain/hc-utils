use crate::error::*;
use crate::local_source_chain::local_source_chain;
use hdk3::prelude::*;

/// Query for an existing Entry in the local source-chain matching the given EntryType name(s).  If
/// one exists, return it Address, otherwise commit it.
pub fn commit_idempotent(entry_id: String, value: Entry) -> UtilsResult<HeaderHash> {
    debug!("Iterate through chain ")?;
    for element in local_source_chain().unwrap().0 {
        if let element::ElementEntry::Present(entry) = element.entry() {
            if entry.clone() == value {
                return Ok(element.header_address().clone());
            }
        }
    }
    debug!("Creating entry...")?;
    let result = create!(EntryDefId::App(entry_id), value)?;
    Ok(result)
}

// TODO
// #[macro_export]
// macro_rules! commit_idempotent {
//     ( $input:expr ) => {
//         // $crate::hdk3::prelude::debug!("Iterate through chain ")?;
//         // for element in $crate::local_source_chain::local_source_chain().unwrap().0 {
//         //     if let $crate::hdk3::prelude::element::ElementEntry::Present(entry) = element.entry() {
//         //         if entry.clone()
//         //             == Entry::App(SerializedBytes::try_from($input)?.try_into().unwrap())
//         //         {
//         //             return Ok(element.header_address().clone());
//         //         }
//         //     }
//         // }
//         match $crate::_commit_idempotent(Entry::App(SerializedBytes::try_from($input)?.try_into().unwrap())) {
//             Ok(header) => return Ok(header),
//             Err(_) => {
//                 debug!("Creating entry...")?;
//                 return $crate::hdk3::prelude::create_entry!($input)
//             }
//         }
//     };
// }
