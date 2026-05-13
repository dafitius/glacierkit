use crate::model::common::Hash;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ContentSearchResultsEvent {
	Initialise { id: Uuid },

	OpenResourceOverview { id: Uuid, hash: Hash }
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ContentSearchResultsRequest {
	Initialise {
		id: Uuid,

		/// Hash, type, path/hint
		#[debug(skip)]
		results: Vec<(String, String, Option<String>)>
	}
}
