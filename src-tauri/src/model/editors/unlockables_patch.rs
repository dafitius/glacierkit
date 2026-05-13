use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;
use crate::ores_repo::UnlockableInformation;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum UnlockablesPatchEditorEvent {
	Initialise { id: Uuid },

	CreateUnlockable { id: Uuid },

	ResetModifications { id: Uuid, unlockable: Uuid },

	ModifyUnlockable { id: Uuid, unlockable: Uuid, data: String },

	SelectUnlockable { id: Uuid, unlockable: Uuid }
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum UnlockablesPatchEditorRequest {
	SetUnlockables {
		id: Uuid,

		#[debug(skip)]
		unlockables: Vec<(Uuid, UnlockableInformation)>
	},

	SetModifiedUnlockables {
		id: Uuid,
		modified: Vec<Uuid>
	},

	AddNewUnlockable {
		id: Uuid,
		new_unlockable: (Uuid, UnlockableInformation)
	},

	RemoveUnlockable {
		id: Uuid,
		unlockable: Uuid
	},

	SetMonacoContent {
		id: Uuid,
		unlockable: Uuid,
		orig_data: String,
		data: String
	},

	DeselectMonaco {
		id: Uuid
	},

	ModifyUnlockableInformation {
		id: Uuid,
		unlockable: Uuid,
		info: UnlockableInformation
	}
}
