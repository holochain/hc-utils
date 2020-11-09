use hdk3::prelude::*;

#[derive(thiserror::Error, Debug)]
pub enum UtilsError {
    #[error(transparent)]
    Serialization(#[from] SerializedBytesError),
    #[error(transparent)]
    EntryError(#[from] EntryError),
    #[error(transparent)]
    Wasm(#[from] WasmError),
    #[error(transparent)]
    HdkError(#[from] HdkError),
    #[error("Agent has not created a profile yet")]
    AgentNotRegisteredProfile,
    // #[error("Header that was just committed is missing. This means something went really wrong")]
    // MissingLocalHeader,
    // #[error("Tried to use a header without an entry as for where it only makes sense to use a new entry header")]
    // WrongHeaderType,
    // #[error("Channel has been deleted too bad")]
    // ChannelDeleted,
    #[error("Unable to find entry in dht")]
    EntryNotFound,
    #[error("Generic\n Error: {0}")]
    Generic(&'static str),
}

pub type UtilsResult<T> = Result<T, UtilsError>;
