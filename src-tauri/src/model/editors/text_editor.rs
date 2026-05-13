use crate::model::common::TextFileType;
use serde::{Deserialize, Serialize};
use specta::Type;
use uuid::Uuid;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum TextEditorEvent {
	Initialise { id: Uuid },
	UpdateContent { id: Uuid, content: String }
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum TextEditorRequest {
	ReplaceContent { id: Uuid, content: String },
	SetFileType { id: Uuid, file_type: TextFileType }
}
