use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ContentSearchEvent {
	Search(String, Vec<String>, bool, Vec<String>)
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ContentSearchRequest {
				SetEnabled(bool),
				SetPartitions(Vec<(String, String)>)
			}