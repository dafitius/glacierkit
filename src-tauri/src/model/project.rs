use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;
use arc_swap::ArcSwap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Project {
	pub path: PathBuf,
	pub settings: ArcSwap<ProjectSettings>
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProjectSettings {
	pub custom_paths: Vec<String>
}

impl Default for ProjectSettings {
	fn default() -> Self {
		Self { custom_paths: vec![] }
	}
}