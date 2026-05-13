use quickentity_rs::entity::EntityID;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;
use ecow::EcoString;
use crate::entity::ReverseReference;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityMetaPaneEvent {
	JumpToReference {
		editor_id: Uuid,
		reference: EntityID
	},

	SetNotes {
		editor_id: Uuid,
		entity_id: EntityID,
		notes: String
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityMetaPaneRequest {
	SetReverseRefs {
		editor_id: Uuid,
		#[specta(type = std::collections::HashMap<EntityID, String>)]
		entity_names: std::collections::HashMap<EntityID, EcoString>,
		reverse_refs: Vec<ReverseReference>
	},

	SetNotes {
		editor_id: Uuid,
		entity_id: EntityID,

		#[specta(type = String)]
		notes: EcoString
	}
}
