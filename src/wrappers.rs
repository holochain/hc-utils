use hdk::prelude::holo_hash::DnaHash;
use hdk::prelude::*;

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
pub struct HashString(String);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct WrappedAgentPubKey(pub AgentPubKey);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct WrappedActionHash(pub ActionHash);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct WrappedEntryHash(pub EntryHash);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct WrappedDnaHash(pub DnaHash);

impl TryFrom<HashString> for WrappedAgentPubKey {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        match AgentPubKey::try_from(ui_string_hash.0) {
            Ok(address) => Ok(Self(address)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}

impl From<WrappedAgentPubKey> for AgentPubKey {
    fn from(ui_string_hash: WrappedAgentPubKey) -> Self {
        return ui_string_hash.0;
    }
}

impl From<WrappedAgentPubKey> for HashString {
    fn from(wrapped_agent_pub_key: WrappedAgentPubKey) -> Self {
        Self(wrapped_agent_pub_key.0.to_string())
    }
}

impl TryFrom<HashString> for WrappedActionHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        match ActionHash::try_from(ui_string_hash.0) {
            Ok(address) => Ok(Self(address)),
            Err(e) => Err(format!("what is this error {:?}", e)),
        }
    }
}
impl From<WrappedActionHash> for HashString {
    fn from(wrapped_action_hash: WrappedActionHash) -> Self {
        Self(wrapped_action_hash.0.to_string())
    }
}

impl TryFrom<HashString> for WrappedEntryHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        match EntryHash::try_from(ui_string_hash.0) {
            Ok(address) => Ok(Self(address)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
impl From<WrappedEntryHash> for HashString {
    fn from(wrapped_entry_hash: WrappedEntryHash) -> Self {
        Self(wrapped_entry_hash.0.to_string())
    }
}

impl TryFrom<HashString> for WrappedDnaHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        match DnaHash::try_from(ui_string_hash.0) {
            Ok(address) => Ok(Self(address)),
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
impl From<WrappedDnaHash> for HashString {
    fn from(wrapped_entry_hash: WrappedDnaHash) -> Self {
        Self(wrapped_entry_hash.0.to_string())
    }
}
