use crate::model::common::Hash;
use serde::{Deserialize, Serialize};
use specta::Type;
use hitman_commons::metadata::ResourceType;
use ecow::EcoString;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
pub enum SearchFilter {
	All,
	Templates,
	Classes,
	Models,
	Textures,
	Sound
}

#[derive(Type, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct GameBrowserEntry {
	pub hash: Hash,
	#[specta(type = Option<String>)]
	pub path: Option<EcoString>,
	#[specta(type = Option<String>)]
	pub hint: Option<EcoString>,
	pub filetype: ResourceType,
	pub partition: (String, String)
}


#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum GameBrowserEvent {
	Select(Hash),
	Search(String, SearchFilter),
	OpenInEditor(Hash)
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum GameBrowserRequest {
	SetEnabled(bool),

	NewTree {
		game_description: String,

		#[debug(skip)]
		entries: Vec<GameBrowserEntry>
	}
}
