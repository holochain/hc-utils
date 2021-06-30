use hdk::prelude::*;
use ::holo_hash::DnaHash;

#[derive(Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
pub struct HashString(String);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct SafeWrappedAgentPubKey(pub AgentPubKey);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct SafeWrappedHeaderHash(pub HeaderHash);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct SafeWrappedEntryHash(pub EntryHash);

#[derive(Hash, Eq, Debug, Serialize, Deserialize, SerializedBytes, Clone, PartialEq)]
#[serde(try_from = "HashString")]
#[serde(into = "HashString")]
pub struct SafeWrappedDnaHash(pub DnaHash);

impl TryFrom<HashString> for SafeWrappedAgentPubKey {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        let mut hash = ui_string_hash.0;
        if &hash[..4] == AGENT_PREFIX {
            hash.replace_range(..4, "");
            assert_eq!(59, hash.len());
            match multibase::decode(hash) {
                Ok((_, bytes)) => Ok(Self(holo_hash::AgentPubKey::from_raw_36_and_type(bytes, holo_hash::hash_type::Agent))),
                Err(e) => Err(format!("{:?}", e)),
            }
        } else {
            Err(format!("Invalid AgentPubKey received {:?}", hash))
        }
    }
}
const AGENT_PREFIX: &str = "hcak";
const HEADER_PREFIX: &str = "hckk";
const ENTRY_PREFIX: &str = "hcek";
const DNA_PREFIX: &str = "hc0k";

impl From<SafeWrappedAgentPubKey> for HashString {
    fn from(wrapped_agent_pub_key: SafeWrappedAgentPubKey) -> Self {
        let hash = multibase::encode(multibase::Base::Base32Lower, &wrapped_agent_pub_key.0.get_raw_39()[3..]);
        Self(format!("{}{}", AGENT_PREFIX, hash.to_string()))
    }
}

impl TryFrom<HashString> for SafeWrappedHeaderHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        let mut hash = ui_string_hash.0;
        if &hash[..4] == HEADER_PREFIX {
            hash.replace_range(..4, "");
            assert_eq!(59, hash.len());
            match multibase::decode(hash) {
                Ok((_, bytes)) => Ok(Self(holo_hash::HeaderHash::from_raw_36_and_type(bytes, holo_hash::hash_type::Header))),
                Err(e) => Err(format!("{:?}", e)),
            }
        } else {
            Err(format!("Invalid HeaderHash received {:?}", hash))
        }
    }
}
impl From<SafeWrappedHeaderHash> for HashString {
    fn from(wrapped_header_hash: SafeWrappedHeaderHash) -> Self {
        let hash = multibase::encode(multibase::Base::Base32Lower, &wrapped_header_hash.0.get_raw_39()[3..]);
        Self(format!("{}{}", HEADER_PREFIX, hash.to_string()))
    }
}

impl TryFrom<HashString> for SafeWrappedEntryHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        let mut hash = ui_string_hash.0;
        if &hash[..4] == ENTRY_PREFIX {
            hash.replace_range(..4, "");
            assert_eq!(59, hash.len());
            match multibase::decode(hash) {
                Ok((_, bytes)) => Ok(Self(holo_hash::EntryHash::from_raw_36_and_type(bytes, holo_hash::hash_type::Entry))),
                Err(e) => Err(format!("{:?}", e)),
            }
        } else {
            Err(format!("Invalid HeaderHash received {:?}", hash))
        }
    }
}
impl From<SafeWrappedEntryHash> for HashString {
    fn from(wrapped_entry_hash: SafeWrappedEntryHash) -> Self {
        let hash = multibase::encode(multibase::Base::Base32Lower, &wrapped_entry_hash.0.get_raw_39()[3..]);
        Self(format!("{}{}", ENTRY_PREFIX, hash.to_string()))
    }
}

impl TryFrom<HashString> for SafeWrappedDnaHash {
    type Error = String;
    fn try_from(ui_string_hash: HashString) -> Result<Self, Self::Error> {
        let mut hash = ui_string_hash.0;
        if &hash[..4] == DNA_PREFIX {
            hash.replace_range(..4, "");
            assert_eq!(59, hash.len());
            match multibase::decode(hash) {
                Ok((_, bytes)) => Ok(Self(holo_hash::DnaHash::from_raw_36_and_type(bytes, holo_hash::hash_type::Dna))),
                Err(e) => Err(format!("{:?}", e)),
            }
        } else {
            Err(format!("Invalid HeaderHash received {:?}", hash))
        }
    }
}
impl From<SafeWrappedDnaHash> for HashString {
    fn from(wrapped_dna_hash: SafeWrappedDnaHash) -> Self {
        let hash = multibase::encode(multibase::Base::Base32Lower, &wrapped_dna_hash.0.get_raw_39()[3..]);
        Self(format!("{}{}", DNA_PREFIX, hash.to_string()))
    }
}
