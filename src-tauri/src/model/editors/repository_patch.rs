use crate::ores_repo::RepositoryItemInformation;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum RepositoryPatchEditorEvent {
	Initialise { id: Uuid },

	CreateRepositoryItem { id: Uuid },

	ResetModifications { id: Uuid, item: Uuid },

	ModifyItem { id: Uuid, item: Uuid, data: String },

	SelectItem { id: Uuid, item: Uuid }
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum RepositoryPatchEditorRequest {
	SetRepositoryItems {
		id: Uuid,

		#[debug(skip)]
		items: Vec<(Uuid, RepositoryItemInformation)>
	},

	SetModifiedRepositoryItems {
		id: Uuid,
		modified: Vec<Uuid>
	},

	AddNewRepositoryItem {
		id: Uuid,
		new_item: (Uuid, RepositoryItemInformation)
	},

	RemoveRepositoryItem {
		id: Uuid,
		item: Uuid
	},

	SetMonacoContent {
		id: Uuid,
		item: Uuid,
		orig_data: String,
		data: String
	},

	DeselectMonaco {
		id: Uuid
	},

	ModifyItemInformation {
		id: Uuid,
		item: Uuid,
		info: RepositoryItemInformation
	}
}
