use hitman_commons::metadata::RuntimeID;
use quickentity_rs::{entity::EntityID, variant::Variant};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;
use ecow::EcoString;


#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type", content = "data")]
pub enum EditorValidity {
	Valid,
	Invalid(String)
}


#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityMonacoEvent {
	UpdateContent {
		editor_id: Uuid,
		entity_id: EntityID,
		content: String
	},

	FollowReference {
		editor_id: Uuid,
		reference: EntityID
	},

	OpenFactory {
		editor_id: Uuid,
		factory: RuntimeID
	},

	SignalPin {
		editor_id: Uuid,
		entity_id: EntityID,
		pin: String,
		output: bool
	},

	OpenResourceOverview {
		editor_id: Uuid,
		resource: RuntimeID
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityMonacoRequest {
	DeselectIfSelected {
		editor_id: Uuid,
		entity_ids: Vec<EntityID>
	},

	ReplaceContent {
		editor_id: Uuid,
		entity_id: EntityID,
		content: String
	},

	ReplaceContentIfSameEntityID {
		editor_id: Uuid,
		entity_id: EntityID,
		content: String
	},

	UpdateIntellisense {
		editor_id: Uuid,
		entity_id: EntityID,
		#[specta(type = Vec<(String, Variant, bool)>)]
		properties: Vec<(EcoString, Variant, bool)>,
		#[specta(type = Vec<String>)]
		input_pins: Vec<EcoString>,
		#[specta(type = Vec<String>)]
		output_pins: Vec<EcoString>
	},

	UpdateDecorationsAndMonacoInfo {
		editor_id: Uuid,
		entity_id: EntityID,
		decorations: Vec<(String, String)>,
		local_ref_entity_ids: Vec<EntityID>
	},

	UpdateValidity {
		editor_id: Uuid,
		validity: EditorValidity
	},

	SetEditorConnected {
		editor_id: Uuid,
		connected: bool
	}
}
