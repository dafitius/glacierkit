pub mod general;
pub mod metadata;
pub mod metapane;
pub mod monaco;
pub mod overrides;
pub mod tree;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::model::editors::entity::{
	general::{EntityGeneralEvent, EntityGeneralRequest}, metadata::{EntityMetadataEvent, EntityMetadataRequest}, metapane::{EntityMetaPaneEvent, EntityMetaPaneRequest},
	monaco::{EntityMonacoEvent, EntityMonacoRequest}, overrides::{EntityOverridesEvent, EntityOverridesRequest}, tree::{EntityTreeEvent, EntityTreeRequest}
};

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityEditorEvent {
	General(EntityGeneralEvent),
	Tree(EntityTreeEvent),
	Monaco(EntityMonacoEvent),
	MetaPane(EntityMetaPaneEvent),
	Metadata(EntityMetadataEvent),
	Overrides(EntityOverridesEvent)
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum EntityEditorRequest {
	General(EntityGeneralRequest),
	Tree(EntityTreeRequest),
	Monaco(EntityMonacoRequest),
	MetaPane(EntityMetaPaneRequest),
	Metadata(EntityMetadataRequest),
	Overrides(EntityOverridesRequest)
}
