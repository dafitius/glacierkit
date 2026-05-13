use hitman_commons::metadata::RuntimeID;
use quickentity_rs::entity::{EntityID, SubType};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityMetadataEvent {
	Initialise {
		editor_id: Uuid
	},

	SetFactory {
		editor_id: Uuid,
		factory: RuntimeID
	},

	SetBlueprint {
		editor_id: Uuid,
		blueprint: RuntimeID
	},

	SetRootEntity {
		editor_id: Uuid,
		root_entity: EntityID
	},

	SetSubType {
		editor_id: Uuid,
		sub_type: SubType
	},

	SetExternalScenes {
		editor_id: Uuid,
		external_scenes: Vec<RuntimeID>
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityMetadataRequest {
	Initialise {
		editor_id: Uuid,
		factory: RuntimeID,
		blueprint: RuntimeID,
		root_entity: EntityID,
		sub_type: SubType,
		external_scenes: Vec<RuntimeID>
	},

	SetHashModificationAllowed {
		editor_id: Uuid,
		hash_modification_allowed: bool
	},

	SetFactory {
		editor_id: Uuid,
		factory: RuntimeID
	},

	SetBlueprint {
		editor_id: Uuid,
		blueprint: RuntimeID
	},

	UpdateCustomPaths {
		editor_id: Uuid,
		custom_paths: Vec<String>
	}
}
