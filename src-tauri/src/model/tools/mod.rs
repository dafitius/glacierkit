pub mod settings;
pub mod file_browser;
pub mod game_browser;
pub mod content_search;

use crate::model::tools::{
			content_search::{ContentSearchEvent, ContentSearchRequest},
			file_browser::{FileBrowserEvent, FileBrowserRequest},
			game_browser::{GameBrowserEvent, GameBrowserRequest},
			settings::{SettingsEvent, SettingsRequest}
		};
use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ToolEvent {
	FileBrowser(FileBrowserEvent),
	GameBrowser(GameBrowserEvent),
	Settings(SettingsEvent),
	ContentSearch(ContentSearchEvent)
}

#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum ToolRequest {
	FileBrowser(FileBrowserRequest),
	GameBrowser(GameBrowserRequest),
	Settings(SettingsRequest),
	ContentSearch(ContentSearchRequest)
}