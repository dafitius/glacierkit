use anyhow::{anyhow, Context, Result};
use arc_swap::ArcSwap;
use fn_error_context::context;
use prim_rs::render_primitive::RenderPrimitive;
use rpkg_rs::GlacierResource;
use specta::Type;
use tauri::{AppHandle, Manager};
use tryvial::try_fn;
use serde::{Serialize, Deserialize};

use crate::{get_loaded_game_version, model::{AppSettings, AppState, EditorData, ModelViewerEvent}, rpkg::extract_latest_resource};

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
pub struct RuntimeRenderPrimitive {
	bb_min: [f32; 3],
	bb_max: [f32; 3],
	wirecolor: [u8; 4],
	lod_mask: u8,
	clothId: u8,
	materialId: u8,
	index_buffer: Vec<u16>,
	position_buffer: Vec<[f32; 3]>,
	normal_buffer: Vec<[f32; 3]>,
	uv_buffer: Vec<Vec<[f32; 3]>>
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
pub struct PrimitiveInstance {
	primitives: Vec<RuntimeRenderPrimitive>,
	lod_mask: u8
}

#[try_fn]
#[context("Couldn't handle Model viewer editor event")]
pub async fn handle(app: &AppHandle, event: ModelViewerEvent) -> Result<()> {

    let app_state = app.state::<AppState>();
    let app_settings = app.state::<ArcSwap<AppSettings>>();

	match event {
		ModelViewerEvent::Initialise { id } => {

            if let Some(game_files) = app_state.game_files.load().as_ref(){
                let editor_state = app_state.editor_states.get(&id).context("No such editor")?;

                let EditorData::ModelViewer { prim_hash, primitive } = editor_state.data.to_owned() else {
                    Err(anyhow!("Editor {} is not a text editor", id))?;
                    panic!();
                };
    
                let (_, res_data) = extract_latest_resource(game_files, prim_hash)?;
                if let Some(game_install) = app_settings.load().game_install.as_ref(){
                    let game_version: hitman_commons::game::GameVersion = get_loaded_game_version(app, game_install).context("Failed to get game version")?;
                    let model = RenderPrimitive::process_data(game_version.into(), res_data)
                        .context("Couldn't process texture data")?;
                }
            };
        },
        ModelViewerEvent::UpdateLod { id, lod } => {todo!()}
	}
}

// },
// model::ModelViewerEvent::UpdateLod { id, lod } => todo!(),

// #[try_fn]
// #[context("Couldn't initialise geometry editor {id}")]
// pub fn initialise_model_viewer(
// 	app: &AppHandle,
// 	app_state: &State<AppState>,
// 	id: Uuid,
// 	hash: RuntimeID,
// 	game_files: &PartitionManager,
// 	game_version: GameVersion,
// 	resource_reverse_dependencies: &Arc<HashMap<RuntimeID, Vec<RuntimeID>>>,
// 	hash_list: &Arc<HashList>
// ) -> Result<()> {
// 	let (filetype, chunk_patch, deps) = extract_latest_overview_info(game_files, hash)?;
// }

// #[try_fn]
// #[context("Couldn't handle model viewer event")]
// pub async fn handle_model_viewer_event(app: &AppHandle, event: RenderPrimitiveEvent) -> Result<()> {
// 	let app_settings = app.state::<ArcSwap<AppSettings>>();
// 	let app_state = app.state::<AppState>();

// 	match event {
// 		ModelViewerEvent::Initialise { id } => {
// 			let editor_state = app_state.editor_states.get(&id).context("No such editor")?;

// 			let hash = match editor_state.data {
// 				EditorData::ModelViewer { prim_hash, primitive } => hash,

// 				_ => {
// 					Err(anyhow!("Editor {} is not a resource overview", id))?;
// 					panic!();
// 				}
// 			};

// 			let task = start_task(app, format!("Loading resource overview for {}", hash))?;

// 			finish_task(app, task)?;
// 		}
// 	}
// }
