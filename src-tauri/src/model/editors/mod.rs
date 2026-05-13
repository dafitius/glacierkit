pub mod text_editor;
pub mod resource_overview;
pub mod repository_patch;
pub mod unlockables_patch;
pub mod entity;
pub mod content_search_results;


use std::path::PathBuf;

use crate::model::editors::{
			content_search_results::{ContentSearchResultsEvent, ContentSearchResultsRequest},
			entity::{EntityEditorEvent, EntityEditorRequest},
			repository_patch::{RepositoryPatchEditorEvent, RepositoryPatchEditorRequest},
			resource_overview::{ResourceOverviewEvent, ResourceOverviewRequest},
			text_editor::{TextEditorEvent, TextEditorRequest},
			unlockables_patch::{UnlockablesPatchEditorEvent, UnlockablesPatchEditorRequest}
		};
use quickentity_rs::entity::Entity;
use hitman_commons::metadata::RuntimeID;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::{
	model::{
		common::{EphemeralQNSettings, JsonPatchType, TextFileType},
    },
    ores_repo::{RepositoryItem, UnlockableItem}
};

#[derive(Debug)]
pub struct EditorState {
	pub file: Option<PathBuf>,
	pub data: EditorData
}

#[derive(Debug, Clone)]
pub enum EditorData {
	Nil,
	ResourceOverview{
		hash: RuntimeID,
	},
	Text {
		content: String,
		file_type: TextFileType
	},
	QNEntity {
		settings: EphemeralQNSettings,
		entity: Box<Entity>
	},
	QNPatch {
		settings: EphemeralQNSettings,
		base: Box<Entity>,
		current: Box<Entity>
	},
	RepositoryPatch {
		base: Vec<RepositoryItem>,
		current: Vec<RepositoryItem>,
		patch_type: JsonPatchType
	},
	UnlockablesPatch {
		base: Vec<UnlockableItem>,
		current: Vec<UnlockableItem>,
		patch_type: JsonPatchType
	},
	ContentSearchResults {
		results: Vec<(String, String, Option<String>)>
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum EditorType {
	Nil,
	ResourceOverview,
	Text { file_type: TextFileType },
	QNEntity,
	QNPatch,
	RepositoryPatch { patch_type: JsonPatchType },
	UnlockablesPatch { patch_type: JsonPatchType },
	ContentSearchResults
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EditorEvent {
	Text(TextEditorEvent),
	Entity(EntityEditorEvent),
	ResourceOverview(ResourceOverviewEvent),
	RepositoryPatch(RepositoryPatchEditorEvent),
	UnlockablesPatch(UnlockablesPatchEditorEvent),
	ContentSearchResults(ContentSearchResultsEvent)
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EditorRequest {
	Text(TextEditorRequest),
	Entity(EntityEditorRequest),
	ResourceOverview(ResourceOverviewRequest),
	RepositoryPatch(RepositoryPatchEditorRequest),
	UnlockablesPatch(UnlockablesPatchEditorRequest),
	ContentSearchResults(ContentSearchResultsRequest)
}