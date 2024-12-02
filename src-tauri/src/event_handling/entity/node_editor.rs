use anyhow::{anyhow, Context, Result};
use arc_swap::{ArcSwap, AsRaw};
use fn_error_context::context;
use hitman_commons::metadata::RuntimeID;
use indexmap::IndexMap;
use quickentity_rs::qn_structs::{Entity, PropertyAlias, Ref, SubEntity};
use serde_json::from_str;
use tauri::{AppHandle, Manager, State};
use tryvial::try_fn;
use uuid::Uuid;
use crate::model::{AppSettings, AppState, EditorData, EditorRequest, EntityEditorRequest, EntityMonacoEvent, EntityNodeData, EntityNodeRefPin, EntityNodeRefPinKind, NodeEditorEvent, NodeEditorRequest, Request};
use crate::rpkg::{extract_entity, extract_latest_metadata};
use crate::{get_loaded_game_version, send_request};

#[try_fn]
#[context("Couldn't handle monaco event")]
pub async fn handle(app: &AppHandle, event: NodeEditorEvent) -> Result<()> {
    let app_state = app.state::<AppState>();

    match event {
        NodeEditorEvent::UpdateContent {
            editor_id,
            entity_id,
        } => {
            let mut editor_state = app_state.editor_states.get_mut(&editor_id).context("No such editor")?;
            let entity = match editor_state.data {
                EditorData::QNEntity { ref mut entity, .. } => entity,
                EditorData::QNPatch { ref mut current, .. } => current,

                _ => {
                    Err(anyhow!("Editor {} is not a QN editor", editor_id))?;
                    panic!();
                }
            };

            let mut entity_path = build_entity_path(entity, &entity_id)?;

            let mut nodes: Vec<EntityNodeData> = get_nodes_from_id(app, entity, &entity_id)?;

            send_request(app, Request::Editor(EditorRequest::Entity(EntityEditorRequest::NodeEditor(NodeEditorRequest::ReplaceContent {
                editor_id,
                entity_id,
                entity_path,
                nodes,
            }))))?;
        }
    }
}

fn get_nodes_from_id(app: &AppHandle, entity: &Entity, entity_id: &String) -> Result<Vec<EntityNodeData>> {
    let root = entity.entities.get(entity_id)
        .with_context(|| format!("No such sub-entity: {}", entity_id))?;



    Ok(entity.entities.iter().filter(|(_, sub_entity)| match &sub_entity.parent {
        Ref::Full(_) => { false }
        Ref::Short(None) => { false }
        Ref::Short(Some(parent)) => { parent == entity_id }
    }).flat_map(|(sub_id, sub_entity)| -> Result<EntityNodeData> {
        let input_pins = get_input_pins(app, &sub_id, &sub_entity)?;
        let output_pins = get_output_pins(app, &sub_id, &sub_entity)?;
        let reference_pins = get_reference_pins(app, &sub_id, &sub_entity)?;

        Ok(EntityNodeData {
        id: sub_id.clone(),
        name: sub_entity.name.clone(),
        entity_type: factory_str_to_typename(&sub_entity.factory),
        input_pins,
        output_pins,
        exposed_entities: get_exposed_entities(&sub_id, &sub_entity),
        reference_pins,
    })}).collect())
}

fn get_input_pins(app: &AppHandle, entity_id: &String, entity: &SubEntity) -> Result<Vec<String>> {

    let mut inputs = vec![];

    let app_settings = app.state::<ArcSwap<AppSettings>>();
    let app_state = app.state::<AppState>();

    if let Some(game_files) = app_state.game_files.load().as_ref()
        && let Some(hash_list) = app_state.hash_list.load().as_ref()
        && let Some(install) = app_settings.load().game_install.as_ref()
    {
        let game_version = get_loaded_game_version(app, install)?;
        let hash = match entity.factory.starts_with("[") {
            true => { RuntimeID::from_path(&entity.factory) }
            false => { RuntimeID::try_from(u64::from_str_radix(&entity.factory, 16)?)? }
        };

        //disable searching cppEntity for now.
        let parent_metadata = extract_latest_metadata(game_files, hash);
        if parent_metadata?.core_info.resource_type == "TEMP"{
            let parent_entity = extract_entity(game_files, &app_state.cached_entities, game_version, hash_list, hash)?;
            let root = parent_entity.entities.get(&parent_entity.root_entity)
                .with_context(|| format!("Can't open root entity"))?;
            if let Some(input_copying) = &root.input_copying{
                inputs.append(&mut input_copying.keys().map(|key| key.clone()).collect());
            }
        }
    }

    if let Some(input_copying) = &entity.input_copying {
        inputs.append(&mut input_copying.keys().map(|key| key.clone()).collect());
    };
    Ok(inputs)
}

fn get_output_pins(app: &AppHandle, entity_id: &String, entity: &SubEntity) -> Result<Vec<String>> {

    let mut outputs = vec![];

    let app_settings = app.state::<ArcSwap<AppSettings>>();
    let app_state = app.state::<AppState>();

    if let Some(game_files) = app_state.game_files.load().as_ref()
        && let Some(hash_list) = app_state.hash_list.load().as_ref()
        && let Some(install) = app_settings.load().game_install.as_ref()
    {
        let game_version = get_loaded_game_version(app, install)?;
        let hash = match entity.factory.starts_with("[") {
            true => { RuntimeID::from_path(&entity.factory) }
            false => { RuntimeID::try_from(u64::from_str_radix(&entity.factory, 16)?)? }
        };

        //disable searching cppEntity for now.
        let parent_metadata = extract_latest_metadata(game_files, hash);
        if parent_metadata?.core_info.resource_type == "TEMP"{
            let parent_entity = extract_entity(game_files, &app_state.cached_entities, game_version, hash_list, hash)?;
            let root = parent_entity.entities.get(&parent_entity.root_entity)
                .with_context(|| format!("Can't open root entity"))?;
            if let Some(output_copying) = &root.output_copying{
                outputs.append(&mut output_copying.keys().map(|key| key.clone()).collect());
            }
        }
    }

    if let Some(output_copying) = &entity.output_copying{
        outputs.append(&mut output_copying.keys().map(|key| key.clone()).collect());
    };
    Ok(outputs)
}

fn get_exposed_entities(entity_id: &str, entity: &SubEntity) -> Vec<String> {
    let mut exposed_entities = vec![];
    exposed_entities.push(entity_id.to_string());
    if let Some(entities) = &entity.exposed_entities {
        exposed_entities.append(&mut entities.keys().map(|id| id.clone()).collect::<Vec<_>>());
    }
    exposed_entities
}

fn get_reference_pins(app: &AppHandle, entity_id: &String, entity: &SubEntity) -> Result<Vec<EntityNodeRefPin>> {
    let mut references = vec![];

    let app_settings = app.state::<ArcSwap<AppSettings>>();
    let app_state = app.state::<AppState>();

    if let Some(game_files) = app_state.game_files.load().as_ref()
        && let Some(hash_list) = app_state.hash_list.load().as_ref()
        && let Some(install) = app_settings.load().game_install.as_ref()
    {
        let game_version = get_loaded_game_version(app, install)?;
        let hash = match entity.factory.starts_with("[") {
            true => { RuntimeID::from_path(&entity.factory) }
            false => { RuntimeID::try_from(u64::from_str_radix(&entity.factory, 16)?)? }
        };

        //disable searching cppEntity for now.
        let parent_metadata = extract_latest_metadata(game_files, hash);
        if parent_metadata?.core_info.resource_type == "TEMP"{
            let parent_entity = extract_entity(game_files, &app_state.cached_entities, game_version, hash_list, hash)?;
            let root = parent_entity.entities.get(&parent_entity.root_entity)
                .with_context(|| format!("Can't open root entity"))?;

            let arrType = "TArray<SEntityTemplateReference>";
            let refType = "SEntityTemplateReference";

            let aliassed_entity = match &root.property_aliases{
                None => {vec![]}
                Some(property_aliases) => {
                    property_aliases.iter()
                        .flat_map(|(alias_name, property_aliases)| {
                            property_aliases.iter().filter_map(|property_alias| {
                                match &property_alias.original_entity {
                                    Ref::Full(_) => None, // TODO: support this
                                    Ref::Short(None) => None,
                                    Ref::Short(Some(id)) => Some((alias_name.clone(), property_alias.original_property.clone(), id.clone())),
                                }
                            })
                        })
                        .collect::<Vec<_>>()
                }
            };

            for alias in aliassed_entity.iter() {
                let alias_entity = parent_entity.entities.get(&alias.2)
                    .with_context(|| format!("No such sub-entity: {}", &alias.2))?;
                if let Some(properties) = &alias_entity.properties{
                    for (property_name, property) in properties.iter() {

                        let kind = if property.property_type == arrType {
                            EntityNodeRefPinKind::RefArray
                        } else if property.property_type == refType {
                            EntityNodeRefPinKind::Ref
                        } else {
                            continue;
                        };

                       if *property_name == alias.1 {
                           references.push(EntityNodeRefPin {
                               name: alias.0.clone(),
                               kind,
                           });
                       }
                    };
                }
            }

            if let Some(properties) = &root.properties{
                for (property_name, property) in properties{
                    let kind = if property.property_type == arrType {
                        EntityNodeRefPinKind::RefArray
                    } else if property.property_type == refType {
                        EntityNodeRefPinKind::Ref
                    } else {
                        continue;
                    };
                    references.push(EntityNodeRefPin {
                        name: property_name.clone(),
                        kind,
                    });
                }
            }
        }
    }
    Ok(references)
}

fn factory_str_to_typename(input: &str) -> String {
    if !input.starts_with('[') {
        return input.to_owned();
    }

    if let Some((before_bracket, _)) = input.rsplit_once(']') {
        let after_sq = match before_bracket.split_once("?/") {
            None => { before_bracket }
            Some((_, after_sq)) => { after_sq }
        };
        if let Some((before_dot, _)) = after_sq.split_once('.') {
            return if let Some((after_slash)) = before_dot.split("/").last() {
                after_slash.to_owned()
            } else {
                before_dot.to_owned()
            };
        }
    }

    input.to_owned()
}


fn build_entity_path(entity: &Entity, entity_id: &String) -> Result<String> {
    let mut path_components = Vec::new();
    let mut current_id = entity_id.clone();

    while current_id != entity.root_entity {
        let sub_entity = entity.entities.get(&current_id)
            .with_context(|| format!("No such sub-entity: {}", current_id))?;

        path_components.push(sub_entity.name.clone());

        match &sub_entity.parent {
            Ref::Full(_) => {
                // If it's an external entity, prepend and stop
                path_components.push("<external entity>".to_string());
                break;
            }
            Ref::Short(Some(parent)) => {
                // Continue with the parent ID
                current_id = parent.to_string();
            }
            Ref::Short(None) => {
                // If there's no parent, stop the traversal
                break;
            }
        }
    }

    let root_entity = entity.entities.get(&current_id)
        .with_context(|| format!("No such sub-entity: {}", current_id))?;

    path_components.push(root_entity.name.clone());

    // Reverse the components to get the correct order and join them
    let entity_path = path_components.into_iter().rev().collect::<Vec<_>>().join("/");

    Ok(entity_path)
}