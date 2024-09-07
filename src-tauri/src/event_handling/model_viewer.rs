use std::borrow::BorrowMut;

use anyhow::{anyhow, Context, Result};
use arc_swap::ArcSwap;
use fn_error_context::context;
use prim_rs::render_primitive::RenderPrimitive;
use rpkg_rs::GlacierResource;
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{AppHandle, Manager};
use tryvial::try_fn;

use crate::{
    EditorRequest,
	get_loaded_game_version,
	model::{AppSettings, AppState, EditorData, ModelViewerEvent, ModelViewerRequest, Request},
	rpkg::extract_latest_resource,
	send_request
};

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
pub struct RuntimeRenderPrimitive {
	bb_min: [f32; 3],
	bb_max: [f32; 3],
	wirecolor: [u8; 4],
	lod_mask: u8,
	clothId: u8,
	material_id: u16,
	index_buffer: Vec<u16>,
	position_buffer: Vec<[f32; 3]>,
	normal_buffer: Vec<[f32; 3]>,
}

impl RuntimeRenderPrimitive {
    fn from_prim_mesh(mesh: &prim_rs::prim_mesh::PrimMesh) -> Self{
        let bb = mesh.sub_mesh.calc_bb();
        Self{
            bb_min: [bb.min.x, bb.min.y, bb.min.z],
            bb_max: [bb.max.x, bb.max.y, bb.max.z],
            wirecolor: mesh.prim_object.wire_color.to_le_bytes(),
            lod_mask: mesh.prim_object.lod_mask,
            clothId: mesh.cloth_id,
            material_id: mesh.prim_object.material_id,
            index_buffer: mesh.sub_mesh.indices.clone(),
            position_buffer: mesh.sub_mesh.buffers.position.iter().map(|pos| [pos.x, pos.y, pos.z]).collect::<Vec<_>>(),
            normal_buffer: mesh.sub_mesh.buffers.main.iter().map(|vert| [vert.normal.x, vert.normal.y, vert.normal.z]).collect::<Vec<_>>(),
        }
    }
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
pub struct PrimitiveInstance {
	lod: u8,
	primitives: Vec<RuntimeRenderPrimitive>
}

impl PrimitiveInstance {
	fn from_lod(primitive: &RenderPrimitive, lod: u8) -> Self {

		// Get only the meshes, we don't need weight metadata for the preview
		let meshes = primitive
			.data
			.objects
			.iter()
			.map(|mesh_obj| mesh_obj.prim_mesh())
			.collect::<Vec<_>>();

		// Get only the meshes for the preferred LOD level
		let meshes = meshes
			.iter()
			.filter(|mesh| mesh.prim_object.lod_mask & (1 << lod) == (1 << lod));

        Self{
            lod,
            primitives: meshes.map(|m| RuntimeRenderPrimitive::from_prim_mesh(*m)).collect::<Vec<_>>(),
        }
	}
}

#[try_fn]
#[context("Couldn't handle Model viewer editor event")]
pub async fn handle(app: &AppHandle, event: ModelViewerEvent) -> Result<()> {
	let app_state = app.state::<AppState>();
	let app_settings = app.state::<ArcSwap<AppSettings>>();

	match event {
		ModelViewerEvent::Initialise { id } => {
			if let Some(game_files) = app_state.game_files.load().as_ref() {
				let mut editor_state = app_state.editor_states.get_mut(&id).context("No such editor")?;

				let EditorData::ModelViewer { prim_hash, .. } = editor_state.data.to_owned() else {
					Err(anyhow!("Editor {} is not a Model viewer", id))?;
					panic!();
				};

				let (_, res_data) = extract_latest_resource(game_files, prim_hash)?;
				if let Some(game_install) = app_settings.load().game_install.as_ref() {
					let game_version: hitman_commons::game::GameVersion =
						get_loaded_game_version(app, game_install).context("Failed to get game version")?;
					let model = RenderPrimitive::process_data(game_version.into(), res_data)
						.context("Couldn't process texture data")?;
					let mut_editor_state = editor_state.borrow_mut();

					let EditorData::ModelViewer { ref mut primitive, .. } = mut_editor_state.data else {
						Err(anyhow!("Editor {} is not a text editor", id))?;
						panic!();
					};
					*primitive = Some(model);

					send_request(
						&app,
						Request::Editor(model::EditorRequest::ModelViewer(ModelViewerRequest::ReplaceGeometry {
							id: id,
							primitive: PrimitiveInstance::from_lod(primitive.unwrap(), 1)
						}))
					)?;
				}
			};
		}
		ModelViewerEvent::UpdateLod { id, lod } => {
			todo!()
		}
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
