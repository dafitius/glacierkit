use anyhow::{anyhow, bail, Context, Result};
use arc_swap::ArcSwap;
use fn_error_context::context;
use glacier_texture::{
    enums::{RenderFormat, TextureType},
    mipblock::MipblockData,
    texture_map::TextureMap,
};
use std::str::FromStr;
use std::{fmt::Write, fs, io::Cursor, ops::Deref, sync::Arc};
use std::env::temp_dir;
use std::path::PathBuf;
use hashbrown::HashMap;
use hitman_commons::{game::GameVersion, hash_list::HashList, metadata::RuntimeID, rpkg_tool::RpkgResourceMeta};
use hitman_commons::metadata::{ExtendedResourceMetadata, ResourceReference};
use hitman_formats::{
    material::{MaterialEntity, MaterialInstance},
    ores::{parse_hashes_ores, parse_json_ores},
    sdef::SoundDefinitions,
    wwev::{WwiseEvent, WwiseEventData},
};
use hitman_formats::material::MaterialPropertyValue;
use image::{ImageFormat, ImageReader};
use log::{log, Level};
use prim_rs::prim_object::{ObjectPropertyFlags, PrimObjectSubtype};
use prim_rs::render_primitive::{PrimHeader, RenderPrimitive};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rpkg_rs::{resource::partition_manager::PartitionManager, GlacierResource};
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, to_vec, Value};
use specta::{Type, TypeMap};
use tauri::{
    api::{dialog::blocking::FileDialogBuilder, process::Command},
    AppHandle, Manager, State,
};
use tauri_plugin_aptabase::EventTracker;
use tonytools::hmlanguages;
use tryvial::try_fn;
use uuid::Uuid;

use crate::model::{GeometryEditorEvent, GeometryEditorRequest};

use crate::{
    biome::format_json,
    finish_task,
    general::open_in_editor,
    get_loaded_game_version,
    languages::get_language_map,
    model::{
        AppSettings, AppState, EditorData, EditorRequest, EditorState, EditorType, GlobalRequest, Request,
        ResourceOverviewData, ResourceOverviewEvent, ResourceOverviewRequest,
    },
    resourcelib::{
        convert_generic, h2016_convert_binary_to_blueprint, h2016_convert_binary_to_factory,
        h2_convert_binary_to_blueprint, h2_convert_binary_to_factory, h3_convert_binary_to_blueprint,
        h3_convert_binary_to_factory,
    },
    rpkg,
    rpkg::{extract_entity, extract_latest_overview_info, extract_latest_resource, extract_resource_changelog},
    send_notification, send_request, start_task, Notification, NotificationKind, RunCommandExt,
};

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub enum GeometryEditorEntryKind {
    Mesh,
    Weighted,
    Linked,
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeometryEditorEntryMaterial {
    name: String,
    textures: Vec<GeometryEditorEntryTexture>,
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeometryEditorEntryTexture {
    name: String,
    path: PathBuf,
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeometryEditorEntryMesh {
    pub lod_mask: u8,
    pub wire_color: u32,
    pub const_color: u32,
    pub material_index: u16,
    pub indices: Vec<u16>,
    pub position_buffer: Vec<(f32, f32, f32)>,
    pub normal_buffer: Vec<(f32, f32, f32)>,
    pub color_buffer: Option<Vec<(f32, f32, f32, f32)>>,
    pub uv_buffer: Vec<(f32, f32)>,
}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeometryEditorEntryCollider {}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeometryEditorEntryRig {}

#[derive(Type, Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeometryEditorObject {
    name: String,
    kind: GeometryEditorEntryKind,
    position: [f32; 3],
    meshes: Vec<GeometryEditorEntryMesh>,
    // colliders: Vec<GeometryEditorEntryCollider>,
    materials: Vec<GeometryEditorEntryMaterial>,
    rig: Option<GeometryEditorEntryRig>,
}

fn get_prim_name(hash_list: Option<&Arc<HashList>>, rrid: RuntimeID) -> Option<String> {
    let path = hash_list?.entries.get(&rrid)?.path.clone()?;
    path.split('/')
        .last()
        .and_then(|segment| segment.split(".prim]").next())
        .and_then(|segment| segment.split(".weightedprim]").next())
        .and_then(|segment| segment.split(".linkedprim]").next())
        .map(str::to_string)
}

#[context("Could convert given RuntimeID to object")]
fn create_geometry_editor_entry(
    app: &AppHandle,
    editor_state: &mut EditorState,
    prim_hash: RuntimeID,
) -> Result<GeometryEditorObject> {
    let app_settings = app.state::<ArcSwap<AppSettings>>();
    let app_state = app.state::<AppState>();
    let prim_name = get_prim_name(app_state.hash_list.load().as_ref(), prim_hash).unwrap_or(format!("{}", prim_hash));

    if let Some(game_files) = app_state.game_files.load().as_ref()
        && let Some(install) = app_settings.load().game_install.as_ref()
    {
        let game_version = get_loaded_game_version(app, install)?;

        let prim_id = Uuid::new_v4();
        let (resource_metadata, prim_data) = rpkg::extract_latest_resource(game_files, prim_hash)?;

        if let EditorData::GeometryEditor { ref mut loaded_prims } = &mut editor_state.data {
            let model = RenderPrimitive::process_data(game_version.into(), prim_data)
                .context("Couldn't process prim data")?;
            loaded_prims.insert(prim_id, model);

            let prim = loaded_prims.get(&prim_id).unwrap();

            let model_flags = prim.data.property_flags.to_owned();
            let mesh_kind = match (model_flags.is_linked_object(), model_flags.is_weighted_object()) {
                (false, false) => GeometryEditorEntryKind::Mesh,
                (true, false) => GeometryEditorEntryKind::Linked,
                (false, true) => GeometryEditorEntryKind::Weighted,
                (true, true) => {
                    panic!("forbidden prim configuration found")
                }
            };

            let meshes = prim
                .data
                .objects
                .iter()
                .map(|object| {
                    let obj_mesh = object.prim_mesh();
                    GeometryEditorEntryMesh {
                        lod_mask: obj_mesh.prim_object.lod_mask,
                        wire_color: obj_mesh.prim_object.wire_color,
                        const_color: obj_mesh.sub_mesh.prim_object.constant_vertex_color,
                        material_index: obj_mesh.sub_mesh.prim_object.material_id,
                        indices: obj_mesh.sub_mesh.indices.to_owned(),
                        position_buffer: obj_mesh
                            .sub_mesh
                            .buffers
                            .position
                            .iter()
                            .map(|pos| (pos.x, pos.y, pos.z))
                            .collect(),
                        normal_buffer: obj_mesh
                            .sub_mesh
                            .buffers
                            .main.iter().map(|main| (main.normal.x, main.normal.y, main.normal.z))
                            .collect(),
                        color_buffer: obj_mesh
                            .sub_mesh
                            .buffers
                            .colors.as_ref()
                            .map(|buffer| buffer.iter().map(|color| (color.r as f32 / 255.0, color.g as f32 / 255.0, color.b as f32 / 255.0, color.a as f32 / 255.0)).collect()),
                        uv_buffer: obj_mesh
                            .sub_mesh
                            .buffers
                            .main.iter().map(|main| (main.uvs[0].x, main.uvs[0].y))
                            .collect(),
                    }
                })
                .collect();

            let data_dir = app.path_resolver().app_data_dir().expect("Couldn't get data dir");
            let materials = get_geometry_editor_materials(game_files, game_version, &data_dir, &resource_metadata.core_info.references);

            return Ok(GeometryEditorObject {
                name: prim_name,
                kind: mesh_kind,
                position: [0.0, 0.0, 0.0],
                meshes,
                materials,
                rig: None,
            });
        } else {
            Err(anyhow!("TODO: make this error message"))?;
            panic!();
        }
    };
    Err(anyhow!("Can't create geometry editor object"))
}

fn get_geometry_editor_materials(game_files: &Arc<PartitionManager>, game_version: GameVersion, data_dir: &PathBuf, references: &Vec<ResourceReference>) -> Vec<GeometryEditorEntryMaterial> {
    let mut ret = vec![];

    for reference in references.iter() {
        if let Ok((reference_metadata, reference_data)) = rpkg::extract_latest_resource(game_files, reference.resource.clone()) { //TODO: Don't extract data before type check
            if reference_metadata.core_info.resource_type.as_ref() == "MATI" {
                if let Ok(material) = MaterialInstance::parse(&reference_data, &reference_metadata.core_info)
                    .context("Couldn't parse material instance") {
                    let name = material.name.to_owned();
                    let textures = material.binder.properties.iter().flat_map(|(key, val)|
                        {
                            match val {
                                MaterialPropertyValue::Float { .. } => {}
                                MaterialPropertyValue::Vector { .. } => {}
                                MaterialPropertyValue::Texture { enabled, value, texture_type, .. } => {
                                    if *enabled && value.is_some() {
                                        return Some((key.to_owned(), value.to_owned().unwrap(), texture_type));
                                    }
                                }
                                MaterialPropertyValue::Colour { .. } => {}
                            }
                            return None;
                        }
                    ).collect::<Vec<_>>();

                    let textures = textures.into_iter().flat_map(|(texture_name, texture_rid, texture_type)| -> Result<GeometryEditorEntryTexture> {
                        let (texture_metadata, texture_resource) = rpkg::extract_latest_resource(game_files, texture_rid)?;
                        let mut texture =
                            TextureMap::process_data(game_version.into(), texture_resource)
                                .context("Couldn't process texture data")?;

                        if let Some(texd_depend) = texture_metadata.core_info.references.first() {
                            let (_, texd_data) =
                                extract_latest_resource(game_files, texd_depend.resource.get_id()).context("Couldn't extract texd resource")?;

                            let mip_block = MipblockData::from_memory(
                                &texd_data,
                                game_version.into(),
                            )
                                .context("Couldn't process TEXD data")?;
                            texture.set_mipblock1(mip_block);
                        }

                        let out_dir = data_dir.join("temp");
                        fs::create_dir_all(&out_dir).context("Failed to create temp dir")?; //Just in case
                        let out_path = out_dir.join(format!("{:?}.png", Uuid::new_v4()));
                        let tga_data = glacier_texture::convert::create_tga(&texture).context("Couldn't create tga")?;
                        let mut reader = ImageReader::new(Cursor::new(tga_data.to_owned()));

                        reader.set_format(image::ImageFormat::Tga);

                        reader
                            .decode().context("Couldn't decode texture")?
                            .save(&out_path).context("Couldn't write texture file to disk")?;
                        
                        log!(Level::Info,"texture exported: {}", out_path.display());
                        
                        return Ok(GeometryEditorEntryTexture {
                            name: texture_name,
                            path: out_path,
                        });
                    }).collect::<Vec<_>>();
                    
                    ret.push(GeometryEditorEntryMaterial {
                        name,
                        textures,
                    })
                }
            }
        }
    }
    ret
}


#[try_fn]
#[context("Couldn't create geometry editor")]
pub fn create_geometry_editor_tab(app: &AppHandle, id: Option<Uuid>) -> Result<()>{
    let app_state = app.state::<AppState>();

    let id = id.unwrap_or_else(|| Uuid::new_v4());

    app_state.editor_states.insert(
        id.to_owned(),
        EditorState {
            file: None,
            data: EditorData::GeometryEditor { loaded_prims: HashMap::new() },
        },
    );

    send_request(
        &app,
        Request::Global(GlobalRequest::CreateTab {
            id,
            name: "Geometry editor".to_string(),
            editor_type: EditorType::GeometryEditor,
        }),
    )?;

    send_request(
        app,
        Request::Editor(EditorRequest::Geometry(GeometryEditorRequest::Initialise { id })),
    )?
}

#[try_fn]
#[context("Couldn't handle geometry editor event")]
pub async fn handle_geometry_editor_event(app: &AppHandle, event: GeometryEditorEvent) -> Result<()> {
    let app_settings = app.state::<ArcSwap<AppSettings>>();
    let app_state = app.state::<AppState>();

    match event {
        GeometryEditorEvent::Initialise => {
            create_geometry_editor_tab(app, None)?;
        }
        GeometryEditorEvent::InitializeWithPrimitive { prim_hash } => {
            let id = Uuid::new_v4();
            create_geometry_editor_tab(app, Some(id))?;

            let mut editor_state = app_state.editor_states.get_mut(&id).context("No such editor")?;
            let object = create_geometry_editor_entry(app, &mut editor_state, prim_hash)?;
            let object_id = Uuid::new_v4();
            send_request(app, Request::Editor(EditorRequest::Geometry(GeometryEditorRequest::AddObject { id, obj_id: object_id, data: object })))?;
        }
        GeometryEditorEvent::AddObjectFromPrimitive { id, prim_hash } => {
            let mut editor_state = app_state.editor_states.get_mut(&id).context("No such editor")?;
            let object = create_geometry_editor_entry(app, &mut editor_state, prim_hash)?;
            let object_id = Uuid::new_v4();
            send_request(app, Request::Editor(EditorRequest::Geometry(GeometryEditorRequest::AddObject { id, obj_id: object_id, data: object })))?;
        }

    }
}
