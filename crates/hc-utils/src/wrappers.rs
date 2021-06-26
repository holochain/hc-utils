use hdk::prelude::*;
use ::holo_hash::DnaHash;
use ::holo_hash::encode::{holo_hash_encode, holo_hash_decode};

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
pub struct HashString(String);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct WrappedAgentPubKey(pub AgentPubKey);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct WrappedHeaderHash(pub HeaderHash);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct WrappedEntryHash(pub EntryHash);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct WrappedDnaHash(pub DnaHash);
pub(crate) const AGENT_PREFIX: &[u8] = &[0x84, 0x20, 0x24]; // uhCAk [132, 32, 36]
pub(crate) const ENTRY_PREFIX: &[u8] = &[0x84, 0x21, 0x24]; // uhCEk [132, 33, 36]
pub(crate) const DNA_PREFIX: &[u8] = &[0x84, 0x2d, 0x24]; // uhC0k [132, 45, 36]
pub(crate) const HEADER_PREFIX: &[u8] = &[0x84, 0x29, 0x24]; // uhCkk [132, 41, 36]

impl TryFrom<HashString> for WrappedAgentPubKey {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        match holo_hash_decode(AGENT_PREFIX, &ui_string_hash.0) {
            Ok(decode) => {
                match AgentPubKey::from_raw_39(decode) {
                    Ok(address) => Ok(Self(address)),
                    Err(e) => Err(format!("{:?}", e)),
                }
            },
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
        Self(holo_hash_encode(wrapped_agent_pub_key.0.get_raw_39()))
    }
}

impl TryFrom<HashString> for WrappedHeaderHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        match holo_hash_decode(HEADER_PREFIX, &ui_string_hash.0) {
            Ok(decode) => {
                match HeaderHash::from_raw_39(decode) {
                    Ok(address) => Ok(Self(address)),
                    Err(e) => Err(format!("{:?}", e)),
                }
            },
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
impl From<WrappedHeaderHash> for HashString {
    fn from(wrapped_header_hash: WrappedHeaderHash) -> Self {
        Self(holo_hash_encode(wrapped_header_hash.0.get_raw_39()))
    }
}

impl TryFrom<HashString> for WrappedEntryHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        match holo_hash_decode(ENTRY_PREFIX, &ui_string_hash.0) {
            Ok(decode) => {
                match EntryHash::from_raw_39(decode) {
                    Ok(address) => Ok(Self(address)),
                    Err(e) => Err(format!("{:?}", e)),
                }
            },
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
impl From<WrappedEntryHash> for HashString {
    fn from(wrapped_entry_hash: WrappedEntryHash) -> Self {
        Self(holo_hash_encode(wrapped_entry_hash.0.get_raw_39()))
    }
}

impl TryFrom<HashString> for WrappedDnaHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        match holo_hash_decode(DNA_PREFIX, &ui_string_hash.0) {
            Ok(decode) => {
                match DnaHash::from_raw_39(decode) {
                    Ok(address) => Ok(Self(address)),
                    Err(e) => Err(format!("{:?}", e)),
                }
            },
            Err(e) => Err(format!("{:?}", e)),
        }
    }
}
impl From<WrappedDnaHash> for HashString {
    fn from(wrapped_dna_hash: WrappedDnaHash) -> Self {
        Self(holo_hash_encode(wrapped_dna_hash.0.get_raw_39()))
    }
}
