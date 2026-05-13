use ecow::EcoString;
use hitman_commons::metadata::{ReferenceFlags, ResourceType, RuntimeID};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;
use crate::model::common::Hash;
use std::path::PathBuf;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResourceChangelogEntry {
	pub operation: ResourceChangelogOperation,
	pub partition: String,
	pub patch: String,
	pub description: String
}

#[derive(Type, Serialize, Deserialize, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ResourceChangelogOperation {
	Delete,
	Init,
	Edit
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ResourceOverviewEvent {
	Initialise { id: Uuid },
	FollowDependency { id: Uuid, new_hash: RuntimeID },
	FollowDependencyInNewTab { id: Uuid, hash: RuntimeID },
	OpenInEditor { id: Uuid },
	ExtractAsQN { id: Uuid },
	ExtractAsFile { id: Uuid },
	ExtractTEMPAsRT { id: Uuid },
	ExtractTBLUAsFile { id: Uuid },
	ExtractTBLUAsRT { id: Uuid },
	ExtractAsRTGeneric { id: Uuid },
	ExtractAsImage { id: Uuid },
	ExtractAsWav { id: Uuid },
	ExtractMultiWav { id: Uuid },
	ExtractSpecificMultiWav { id: Uuid, index: u32 },
	ExtractORESAsJson { id: Uuid },
	ExtractAsHMLanguages { id: Uuid }
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ResourceOverviewRequest {
	Initialise {
		id: Uuid,
		hash: Hash,
		filetype: String,
		chunk_patch: String,

		#[specta(type = Option<String>)]
		path_or_hint: Option<EcoString>,

		/// Hash, type, path/hint, flags, is actually in current game version
		#[debug(skip)]
		#[specta(type = Vec<(String, String, Option<String>, ReferenceFlags, bool)>)]
		dependencies: Vec<(Hash, Option<ResourceType>, Option<EcoString>, ReferenceFlags, bool)>,

		/// Hash, type, path/hint
		#[debug(skip)]
		#[specta(type = Vec<(String, String, Option<String>)>)]
		reverse_dependencies: Vec<(Hash, ResourceType, Option<EcoString>)>,

		changelog: Vec<ResourceChangelogEntry>,

		data: ResourceOverviewData
	}
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(tag = "type", content = "data")]
pub enum ResourceOverviewData {
	Generic,
	Entity {
		blueprint_hash: Hash,
		#[specta(type = Option<String>)]
		blueprint_path_or_hint: Option<EcoString>
	},
	GenericRL {
		json: String
	},
	Json {
		json: String
	},
	Ores {
		json: String
	},
	Image {
		image_path: PathBuf,
		dds_data: Option<(String, String)>
	},
	Audio {
		wav_path: PathBuf
	},
	Mesh {
		#[debug(skip)]
		obj: String,
		bounding_box: [f32; 6]
	},
	MultiAudio {
		name: String,
		wav_paths: Vec<(String, PathBuf)>
	},
	Repository,
	Unlockables,
	HMLanguages {
		json: String
	},
	LocalisedLine {
		languages: Vec<(String, String)>
	},
	MaterialInstance {
		json: String
	},
	MaterialEntity {
		json: String
	},
	SoundDefinitions {
		json: String
	}
}

