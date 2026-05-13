use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum FileBrowserEvent {
	Select(Option<PathBuf>),
	Create { path: PathBuf, is_folder: bool },
	Delete(PathBuf),
	Rename { old_path: PathBuf, new_path: PathBuf },
	NormaliseQNFile { path: PathBuf },
	ConvertEntityToPatch { path: PathBuf },
	ConvertPatchToEntity { path: PathBuf },
	ConvertRepoPatchToMergePatch { path: PathBuf },
	ConvertRepoPatchToJsonPatch { path: PathBuf },
	ConvertUnlockablesPatchToMergePatch { path: PathBuf },
	ConvertUnlockablesPatchToJsonPatch { path: PathBuf }
}


#[derive(Type, Serialize, Deserialize, Clone, derive_more::Debug)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum FileBrowserRequest {
	Create {
		path: PathBuf,
		is_folder: bool
	},

	Delete(PathBuf),

	Rename {
		old_path: PathBuf,
		new_path: PathBuf
	},

	BeginRename {
		old_path: PathBuf
	},

	FinishRename {
		new_path: PathBuf
	},

	Select(Option<PathBuf>),

	NewTree {
		base_path: PathBuf,

		/// Relative path, is folder
		#[debug(skip)]
		files: Vec<(PathBuf, bool)>
	}
}
