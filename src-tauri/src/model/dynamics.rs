use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Type, Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Dynamics {
	pub announcements: Vec<Announcement>
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
	pub id: String,
	pub kind: AnnouncementKind,
	pub title: String,
	pub description: String,
	pub persistent: bool,
	pub until: Option<u32>
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AnnouncementKind {
	Info,
	Success,
	Warning,
	Error
}