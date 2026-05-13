use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;
use hitman_commons::game_detection::GameInstall;
use crate::model::project::ProjectSettings;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
	pub extract_modded_files: bool,
	pub game_install: Option<PathBuf>,
	pub colourblind_mode: bool,
	pub editor_connection: bool,
	pub seen_announcements: Vec<String>
}

impl Default for AppSettings {
	fn default() -> Self {
		Self {
			extract_modded_files: false,
			game_install: None,
			colourblind_mode: false,
			editor_connection: true,
			seen_announcements: vec![]
		}
	}
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum SettingsEvent {
	Initialise,
	ChangeGameInstall(Option<PathBuf>),
	ChangeExtractModdedFiles(bool),
	ChangeColourblind(bool),
	ChangeEditorConnection(bool),
	ChangeCustomPaths(Vec<String>)
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum SettingsRequest {
	Initialise {
		game_installs: Vec<GameInstall>,
		settings: AppSettings
	},
	ChangeProjectSettings(ProjectSettings)
}
