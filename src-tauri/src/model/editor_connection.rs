use crate::model::common::Hash;
use quickentity_rs::{
	entity::EntityID,
	variant::{Transform, Variant}
};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EditorConnectionEvent {
	// Entity ID, TBLU hash
	EntitySelected(EntityID, Hash),

	// Entity ID, TBLU hash, transform
	EntityTransformUpdated(EntityID, Hash, Transform),

	// Entity ID, TBLU hash, property name, property type, new value
	EntityPropertyChanged(EntityID, Hash, String, Variant)
}
