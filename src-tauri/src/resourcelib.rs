use anyhow::{bail, Context, Result};
use fn_error_context::context;
use hitman_commons::game::GameVersion;
use hitman_commons::metadata::ResourceType;
use hitman_commons::resourcelib::{
    EntityBlueprint, EntityBlueprintLegacy, EntityFactory, EntityFactoryLegacy, Property
};
use resourcelib_ffi::{ResourceConverter, ResourceGenerator, WoaVersion};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tryvial::try_fn;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SCppEntity {
    pub blueprint_index_in_resource_header: i32,
    pub property_values: Vec<Property>
}

#[derive(Serialize, Deserialize)]
pub struct SwitchGroup {
    pub m_aSwitches: Vec<String>
}


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SExtendedCppEntityBlueprint {
    pub properties: Vec<SExtendedCppEntityProperty>
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SExtendedCppEntityProperty {
    pub property_name: String,
    pub property_type: EExtendedPropertyType,
    pub rt_editable: bool,
    pub extra_data: u64
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum EExtendedPropertyType {
    TYPE_RESOURCEPTR,
    TYPE_INT32,
    TYPE_UINT32,
    TYPE_FLOAT,
    TYPE_STRING,
    TYPE_BOOL,
    TYPE_ENTITYREF,
    TYPE_VARIANT
}


#[derive(Serialize, Deserialize)]
pub struct SUIControlBlueprint {
    pub m_aAttributes: Vec<SAttributeInfo>,
    pub m_aSpecialMethods: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct SAttributeInfo {
    pub m_eKind: EAttributeKind,
    pub m_eType: EAttributeType,
    pub m_sName: String
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum EAttributeKind {
    E_ATTRIBUTE_KIND_PROPERTY,
    E_ATTRIBUTE_KIND_INPUT_PIN,
    E_ATTRIBUTE_KIND_OUTPUT_PIN
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum EAttributeType {
    E_ATTRIBUTE_TYPE_VOID,
    E_ATTRIBUTE_TYPE_INT,
    E_ATTRIBUTE_TYPE_FLOAT,
    E_ATTRIBUTE_TYPE_STRING,
    E_ATTRIBUTE_TYPE_BOOL,
    E_ATTRIBUTE_TYPE_ENTITYREF,
    E_ATTRIBUTE_TYPE_OBJECT
}

fn get_woa_version(game_version: &GameVersion) -> WoaVersion {
    match game_version{
        GameVersion::H1 => {WoaVersion::HM2016}
        GameVersion::H2 => {WoaVersion::HM2}
        GameVersion::H3 => {WoaVersion::HM3}
    }
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TEMP")]
pub fn convert_binary_to_factory(game_version: GameVersion, data: &[u8]) -> Result<EntityFactory> {
    match game_version {
        GameVersion::H1 => {
            convert_generic::<EntityFactoryLegacy>(data, game_version, ResourceType::try_from("TEMP")?)?.into_modern()
        }
        _ => {
            convert_generic::<EntityFactory>(data, game_version, ResourceType::try_from("TEMP")?)?
        }
    }
}

#[try_fn]
#[context("Couldn't convert ResourceLib TEMP to binary data")]
pub fn convert_factory_to_binary(game_version: GameVersion, data: &EntityFactory) -> Result<Vec<u8>> {
    match game_version {
        GameVersion::H1 => {
            generate_generic::<EntityFactoryLegacy>(&data.to_owned().into_legacy(), game_version, ResourceType::try_from("TEMP")?)?
        }
        _ => {
            generate_generic::<EntityFactory>(data, game_version, ResourceType::try_from("TEMP")?)?
        }
    }
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TBLU")]
pub fn convert_binary_to_blueprint(game_version: GameVersion, data: &[u8]) -> Result<EntityBlueprint> {
    match game_version {
        GameVersion::H1 => {
            convert_generic::<EntityBlueprintLegacy>(data, game_version, ResourceType::try_from("TBLU")?)?.into_modern()
        }
        _ => {
            convert_generic::<EntityBlueprint>(data, game_version, ResourceType::try_from("TBLU")?)?
        }
    }
}

#[try_fn]
#[context("Couldn't convert ResourceLib TBLU to binary data")]
pub fn convert_blueprint_to_binary(game_version: GameVersion, data: &EntityBlueprint) -> Result<Vec<u8>> {
    match game_version {
        GameVersion::H1 => {
            generate_generic::<EntityBlueprintLegacy>(&data.to_owned().into_legacy(), game_version, ResourceType::try_from("TBLU")?)?
        }
        _ => {
            generate_generic::<EntityBlueprint>(data, game_version, ResourceType::try_from("TBLU")?)?
        }
    }
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib CPPT")]
pub fn convert_cppt(game_version: GameVersion, data: &[u8]) -> Result<SCppEntity> {
    convert_generic::<SCppEntity>(data, game_version, ResourceType::try_from("CPPT")?)?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib DSWB")]
pub fn convert_dswb(game_version: GameVersion, data: &[u8]) -> Result<SwitchGroup> {
    convert_generic::<SwitchGroup>(data, game_version, ResourceType::try_from("DSWB")?)?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib WSGB")]
pub fn convert_wsgb(game_version: GameVersion, data: &[u8]) -> Result<SwitchGroup> {
    convert_generic::<SwitchGroup>(data, game_version, ResourceType::try_from("WSGB")?)?
}


#[try_fn]
#[context("Couldn't convert binary data to ResourceLib ECPB")]
pub fn convert_ecpb(game_version: GameVersion, data: &[u8]) -> Result<SExtendedCppEntityBlueprint> {
    convert_generic::<SExtendedCppEntityBlueprint>(data, game_version, ResourceType::try_from("ECPB")?)?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib UICB")]
pub fn convert_uicb(game_version: GameVersion, data: &[u8]) -> Result<SUIControlBlueprint> {
    convert_generic::<SUIControlBlueprint>(data, game_version, ResourceType::try_from("UICB")?)?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib format")]
pub fn convert_generic<T: DeserializeOwned>(data: &[u8], game: GameVersion, resource_type: ResourceType) -> Result<T> {
    let resource_type_str = String::from_utf8(resource_type.into()).context("Couldn't convert resource type to string")?;
    let converter = ResourceConverter::new(get_woa_version(&game), &resource_type_str).context(format!("Couldn't create resource converter for {}", resource_type_str))?;
    let json_string = converter.memory_to_json_string(data).context("Couldn't convert data to JsonString")?;
    serde_json::from_str(&json_string).context("Couldn't deserialise returned JsonString as requested type")?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib format")]
pub fn convert_generic_str(data: &[u8], game: GameVersion, resource_type: ResourceType) -> Result<String> {
    convert_generic::<String>(data, game, resource_type)?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib format")]
pub fn generate_generic<T: Serialize>(data: &T, game: GameVersion, resource_type: ResourceType) -> Result<Vec<u8>> {
    let resource_type_str = String::from_utf8(resource_type.into()).context("Couldn't convert resource type to string")?;
    let generator = ResourceGenerator::new(get_woa_version(&game), &resource_type_str).context(format!("Couldn't create resource generator for {}", resource_type_str))?;
    let data = serde_json::to_string(data).context("Couldn't serialize given struct")?;
    generator.json_string_to_resource_mem(&data, false).context("Couldn't convert data to memory")?
}