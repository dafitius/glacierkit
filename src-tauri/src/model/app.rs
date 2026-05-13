use std::{path::PathBuf, sync::Arc};

use arc_swap::ArcSwapOption;
use dashmap::DashMap;
use hashbrown::HashMap;
use hitman_commons::{
	game_detection::GameInstall,
	metadata::{ResourceType, RuntimeID}
};

use notify::RecommendedWatcher;
use notify_debouncer_full::FileIdMap;
use quickentity_rs::entity::Entity;
use rpkg_rs::resource::partition_manager::PartitionManager;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use specta::Type;
use uuid::Uuid;

use crate::{
	editor_connection::EditorConnection,
	intellisense::Intellisense,
	model::{
		dynamics::Dynamics,
		editor_connection::EditorConnectionEvent,
		editors::{EditorState, EditorType, EditorEvent, EditorRequest},
		project::Project,
		tools::{ToolEvent, ToolRequest}
	},
	ores_repo::RepositoryItem
};

pub struct AppState {
	pub game_installs: Vec<GameInstall>,
	pub project: ArcSwapOption<Project>,
	pub tonytools_hash_list: ArcSwapOption<tonytools::hashlist::HashList>,
	pub fs_watcher: ArcSwapOption<notify_debouncer_full::Debouncer<RecommendedWatcher, FileIdMap>>,
	pub editor_states: Arc<DashMap<Uuid, EditorState>>,
	pub game_files: ArcSwapOption<PartitionManager>,

	/// Resource -> Resources which depend on it
	pub resource_reverse_dependencies: ArcSwapOption<HashMap<RuntimeID, Vec<RuntimeID>>>,
	pub file_types: ArcSwapOption<HashMap<RuntimeID, ResourceType>>,

	pub cached_entities: Arc<DashMap<RuntimeID, Entity>>,
	pub repository: ArcSwapOption<Vec<RepositoryItem>>,
	pub intellisense: ArcSwapOption<Intellisense>,

	pub editor_connection: EditorConnection
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum GlobalEvent {
	SetSeenAnnouncements(Vec<String>),
	LoadWorkspace(PathBuf),
	SelectAndOpenFile,
	SelectTab(Option<Uuid>),
	RemoveTab(Uuid),
	SaveTab(Uuid),
	UploadLogAndReport(String),
	UploadLastPanic,
	ClearLastPanic
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum GlobalRequest {
	ErrorReport {
		error: String
	},
	SetWindowTitle(String),
	InitialiseDynamics {
		dynamics: Dynamics,
		seen_announcements: Vec<String>
	},
	CreateTab {
		id: Uuid,
		name: String,
		editor_type: EditorType
	},
	RenameTab {
		id: Uuid,
		new_name: String
	},
	SelectTab(Uuid),
	SetTabUnsaved {
		id: Uuid,
		unsaved: bool
	},
	RemoveTab(Uuid),
	ComputeJSONPatchAndSave {
		base: Value,
		current: Value,
		save_path: PathBuf,
		file_and_type: (String, String)
	},
	RequestLastPanicUpload,
	LogUploadRejected
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum Event {
	Tool(ToolEvent),
	Editor(EditorEvent),
	Global(GlobalEvent),
	EditorConnection(EditorConnectionEvent)
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum Request {
	Tool(ToolRequest),
	Editor(EditorRequest),
	Global(GlobalRequest)
}
