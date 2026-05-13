use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityOverridesEvent {
	Initialise { editor_id: Uuid },
	UpdatePropertyOverrides { editor_id: Uuid, content: String },
	UpdateOverrideDeletes { editor_id: Uuid, content: String },
	UpdatePinConnectionOverrides { editor_id: Uuid, content: String },
	UpdatePinConnectionOverrideDeletes { editor_id: Uuid, content: String }
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityOverridesRequest {
	Initialise {
		editor_id: Uuid,
		property_overrides: String,
		override_deletes: String,
		pin_connection_overrides: String,
		pin_connection_override_deletes: String
	},

	UpdateDecorations {
		editor_id: Uuid,
		decorations: Vec<(String, String)>
	}
}
