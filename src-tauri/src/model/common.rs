use hitman_commons::metadata::RuntimeID;
use serde::{Deserialize, Serialize};
use specta::Type;


#[derive(Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(from = "HashProxy", into = "HashProxy")]
pub struct Hash(pub RuntimeID);

struct HashProxy(RuntimeID);

impl From<Hash> for HashProxy {
	fn from(value: Hash) -> Self {
		Self(value.0)
	}
}

impl From<HashProxy> for Hash {
	fn from(value: HashProxy) -> Self {
		Self(value.0)
	}
}

impl Serialize for HashProxy {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer
	{
		serializer.serialize_str(&self.0.to_hash())
	}
}

impl<'de> Deserialize<'de> for HashProxy {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
	where
		D: serde::Deserializer<'de>
	{
		use serde::de::Error;

		String::deserialize(deserializer)?
			.parse()
			.map_err(D::Error::custom)
			.map(Self)
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
pub enum JsonPatchType {
	MergePatch,
	JsonPatch
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
pub enum TextFileType {
	Json,
	ManifestJson,
	PlainText,
	Markdown
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EphemeralQNSettings {
	pub show_reverse_parent_refs: bool,
	pub show_changes_from_original: bool
}

impl Default for EphemeralQNSettings {
	fn default() -> Self {
		Self {
			show_reverse_parent_refs: false,
			show_changes_from_original: false
		}
	}
}