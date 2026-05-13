use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityGeneralEvent {
	SetShowReverseParentRefs {
		editor_id: Uuid,
		show_reverse_parent_refs: bool
	},

	SetShowChangesFromOriginal {
		editor_id: Uuid,
		show_changes_from_original: bool
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityGeneralRequest {
	SetIsPatchEditor { editor_id: Uuid, is_patch_editor: bool }
}
