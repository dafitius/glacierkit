use crate::{entity::CopiedEntityData, model::common::Hash};
use quickentity_rs::entity::{EntityID, SubEntity, Ref};
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;
use ecow::EcoString;
use hitman_commons::metadata::RuntimeID;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PastableTemplate {
	pub name: String,
	pub icon: String,
	pub paste_data: CopiedEntityData
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PastableTemplateCategory {
	pub name: String,
	pub icon: String,
	pub templates: Vec<PastableTemplate>
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityTreeEvent {
	Initialise {
		editor_id: Uuid
	},

	Select {
		editor_id: Uuid,
		id: EntityID
	},

	Create {
		editor_id: Uuid,
		id: EntityID,
		content: SubEntity
	},

	Delete {
		editor_id: Uuid,
		id: EntityID
	},

	Rename {
		editor_id: Uuid,
		id: EntityID,
		new_name: String
	},

	Reparent {
		editor_id: Uuid,
		id: EntityID,
		new_parent: Option<EntityID>
	},

	Copy {
		editor_id: Uuid,
		id: EntityID
	},

	Paste {
		editor_id: Uuid,
		parent_id: String
	},

	Search {
		editor_id: Uuid,
		query: String
	},

	ShowHelpMenu {
		editor_id: Uuid,
		entity_id: EntityID
	},

	UseTemplate {
		editor_id: Uuid,
		parent_id: String,
		template: CopiedEntityData
	},

	AddGameBrowserItem {
		editor_id: Uuid,
		parent_id: String,
		file: Hash
	},

	SelectEntityInEditor {
		editor_id: Uuid,
		entity_id: EntityID
	},

	MoveEntityToPlayer {
		editor_id: Uuid,
		entity_id: EntityID
	},

	RotateEntityAsPlayer {
		editor_id: Uuid,
		entity_id: EntityID
	},

	MoveEntityToCamera {
		editor_id: Uuid,
		entity_id: EntityID
	},

	RotateEntityAsCamera {
		editor_id: Uuid,
		entity_id: EntityID
	},

	RestoreToOriginal {
		editor_id: Uuid,
		entity_id: EntityID
	}
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityTreeRequest {
	/// Will trigger a Select event from the tree - ensure this doesn't end up in a loop
	Select {
		editor_id: Uuid,
		id: Option<EntityID>
	},

	NewTree {
		editor_id: Uuid,

		/// ID, parent, name, factory, has reverse parent refs
		#[debug(skip)]
		#[specta(type = Vec<(EntityID, Option<Ref>, String, RuntimeID, bool)>)]
		entities: Vec<(EntityID, Option<Ref>, EcoString, RuntimeID, bool)>
	},

	/// Instructs the frontend to take the list of new entities, add any new ones and update any ones that already exist (by ID) with the new information.
	/// This is used for pasting, and for ensuring that icons/parent status/name are updated when a sub-entity is updated.
	NewItems {
		editor_id: Uuid,

		/// ID, parent, name, factory, has reverse parent refs
		#[debug(skip)]
		#[specta(type = Vec<(EntityID, Option<Ref>, String, RuntimeID, bool)>)]
		new_entities: Vec<(EntityID, Option<Ref>, EcoString, RuntimeID, bool)>
	},

	SearchResults {
		editor_id: Uuid,

		/// The IDs of the entities matching the query
		#[debug(skip)]
		results: Vec<EntityID>
	},

	ShowHelpMenu {
		editor_id: Uuid,
		factory: RuntimeID,
		#[specta(type = Vec<String>)]
		input_pins: Vec<EcoString>,
		#[specta(type = Vec<String>)]
		output_pins: Vec<EcoString>,
		default_properties_json: String
	},

	SetTemplates {
		editor_id: Uuid,
		templates: Vec<PastableTemplateCategory>
	},

	SetEditorConnectionAvailable {
		editor_id: Uuid,
		editor_connection_available: bool
	},

	SetShowDiff {
		editor_id: Uuid,
		show_diff: bool
	},

	SetDiffInfo {
		editor_id: Uuid,
		new: Vec<EntityID>,
		modified: Vec<EntityID>,
		#[specta(type = Vec<(EntityID, Option<Ref>, String, RuntimeID, bool)>)]
		removed: Vec<(EntityID, Option<Ref>, EcoString, RuntimeID, bool)>
	}
}
