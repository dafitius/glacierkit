use std::ffi::{CStr, CString};

use anyhow::{bail, Context, Result};
use fn_error_context::context;
use hitman_commons::game::GameVersion;
use hitman_commons::metadata::ResourceType;
use hitman_commons::resourcelib::{
	EntityBlueprint, EntityBlueprintLegacy, EntityFactory, EntityFactoryLegacy, Property
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use tryvial::try_fn;

use resourcelib_ffi::{ResourceConverter, ResourceGenerator};

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TEMP")]
pub fn h3_convert_binary_to_factory(data: &[u8]) -> Result<EntityFactory> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM3, "TEMP")
		.context("Creating a ResourceConverter for TEMP resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given TEMP data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as EntityFactory")?;
	res
}

#[try_fn]
#[context("Couldn't convert ResourceLib TEMP to binary data")]
pub fn h3_convert_factory_to_binary(data: &EntityFactory) -> Result<Vec<u8>> {
	let generator = ResourceGenerator::new(resourcelib_ffi::WoaVersion::HM3, "TEMP")
		.context("Creating a ResourceGenerator for TEMP resources")?;

	let json_string = serde_json::to_string(data)?;

	generator
		.json_string_to_resource_mem(json_string.as_str(), false)
		.context("Failed to convert json string to binary resource")?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TBLU")]
pub fn h3_convert_binary_to_blueprint(data: &[u8]) -> Result<EntityBlueprint> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM3, "TBLU")
		.context("Creating a ResourceConverter for TBLU resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given TBLU data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as EntityBlueprint")?;
	res
}

#[try_fn]
#[context("Couldn't convert ResourceLib TBLU to binary data")]
pub fn h3_convert_blueprint_to_binary(data: &EntityBlueprint) -> Result<Vec<u8>> {
	let generator = ResourceGenerator::new(resourcelib_ffi::WoaVersion::HM3, "TBLU")
		.context("Creating a ResourceGenerator for TBLU resources")?;

	let json_string = serde_json::to_string(data)?;

	generator
		.json_string_to_resource_mem(json_string.as_str(), false)
		.context("Failed to convert json string to binary resource")?
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SCppEntity {
	pub blueprint_index_in_resource_header: i32,
	pub property_values: Vec<Property>
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib CPPT")]
pub fn h3_convert_cppt(data: &[u8]) -> Result<SCppEntity> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM3, "CPPT")
		.context("Creating a ResourceConverter for CPPT resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given CPPT data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SCppEntity")?;
	res
}

#[derive(Serialize, Deserialize)]
pub struct SwitchGroup {
	pub m_aSwitches: Vec<String>
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib DSWB")]
pub fn h3_convert_dswb(data: &[u8]) -> Result<SwitchGroup> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM3, "DSWB")
		.context("Creating a ResourceConverter for DSWB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given DSWB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SwitchGroup")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib WSGB")]
pub fn h3_convert_wsgb(data: &[u8]) -> Result<SwitchGroup> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM3, "WSGB")
		.context("Creating a ResourceConverter for WSGB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given WSGB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SwitchGroup")?;
	res
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

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib ECPB")]
pub fn h3_convert_ecpb(data: &[u8]) -> Result<SExtendedCppEntityBlueprint> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM3, "ECPB")
		.context("Creating a ResourceConverter for ECPB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given ECPB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SExtendedCppEntityBlueprint")?;
	res
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

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib UICB")]
pub fn convert_uicb(data: &[u8]) -> Result<SUIControlBlueprint> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM3, "UICB")
		.context("Creating a ResourceConverter for UICB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given UICB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SUIControlBlueprint")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TEMP")]
pub fn h2_convert_binary_to_factory(data: &[u8]) -> Result<EntityFactory> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2, "TEMP")
		.context("Creating a ResourceConverter for TEMP resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given TEMP data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as EntityFactory")?;
	res
}

#[try_fn]
#[context("Couldn't convert ResourceLib TEMP to binary data")]
pub fn h2_convert_factory_to_binary(data: &EntityFactory) -> Result<Vec<u8>> {
	let generator = ResourceGenerator::new(resourcelib_ffi::WoaVersion::HM2, "TEMP")
		.context("Creating a ResourceGenerator for TEMP resources")?;

	let json_string = serde_json::to_string(data)?;

	generator
		.json_string_to_resource_mem(json_string.as_str(), false)
		.context("Failed to convert json string to binary resource")?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TBLU")]
pub fn h2_convert_binary_to_blueprint(data: &[u8]) -> Result<EntityBlueprint> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2, "TBLU")
		.context("Creating a ResourceConverter for TBLU resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given TBLU data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as EntityBlueprint")?;
	res
}

#[try_fn]
#[context("Couldn't convert ResourceLib TBLU to binary data")]
pub fn h2_convert_blueprint_to_binary(data: &EntityBlueprint) -> Result<Vec<u8>> {
	let generator = ResourceGenerator::new(resourcelib_ffi::WoaVersion::HM2, "TBLU")
		.context("Creating a ResourceGenerator for TBLU resources")?;

	let json_string = serde_json::to_string(data)?;

	generator
		.json_string_to_resource_mem(json_string.as_str(), false)
		.context("Failed to convert json string to binary resource")?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib CPPT")]
pub fn h2_convert_cppt(data: &[u8]) -> Result<SCppEntity> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2, "CPPT")
		.context("Creating a ResourceConverter for CPPT resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given CPPT data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SCppEntity")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib DSWB")]
pub fn h2_convert_dswb(data: &[u8]) -> Result<SwitchGroup> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2, "DSWB")
		.context("Creating a ResourceConverter for DSWB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given DSWB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SwitchGroup")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib WSGB")]
pub fn h2_convert_wsgb(data: &[u8]) -> Result<SwitchGroup> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2, "WSGB")
		.context("Creating a ResourceConverter for WSGB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given WSGB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SUIControlBlueprint")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib ECPB")]
pub fn h2_convert_ecpb(data: &[u8]) -> Result<SExtendedCppEntityBlueprint> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2, "ECPB")
		.context("Creating a ResourceConverter for ECPB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given ECPB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SExtendedCppEntityBlueprint")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TEMP")]
pub fn h2016_convert_binary_to_factory(data: &[u8]) -> Result<EntityFactoryLegacy> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2016, "TEMP")
		.context("Creating a ResourceConverter for TEMP resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given TEMP data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as EntityFactoryLegacy")?;
	res
}

#[try_fn]
#[context("Couldn't convert ResourceLib TEMP to binary data")]
pub fn h2016_convert_factory_to_binary(data: &EntityFactoryLegacy) -> Result<Vec<u8>> {
	let generator = ResourceGenerator::new(resourcelib_ffi::WoaVersion::HM2016, "TEMP")
		.context("Creating a ResourceGenerator for TEMP resources")?;

	let json_string = serde_json::to_string(data)?;

	generator
		.json_string_to_resource_mem(json_string.as_str(), false)
		.context("Failed to convert json string to binary resource")?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib TBLU")]
pub fn h2016_convert_binary_to_blueprint(data: &[u8]) -> Result<EntityBlueprintLegacy> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2016, "TBLU")
		.context("Creating a ResourceConverter for TBLU resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given TBLU data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as EntityBlueprintLegacy")?;
	res
}

#[try_fn]
#[context("Couldn't convert ResourceLib TBLU to binary data")]
pub fn h2016_convert_blueprint_to_binary(data: &EntityBlueprintLegacy) -> Result<Vec<u8>> {
	let generator = ResourceGenerator::new(resourcelib_ffi::WoaVersion::HM2016, "TBLU")
		.context("Creating a ResourceGenerator for TBLU resources")?;

	let json_string = serde_json::to_string(data)?;

	generator
		.json_string_to_resource_mem(json_string.as_str(), false)
		.context("Failed to convert json string to binary resource")?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib CPPT")]
pub fn h2016_convert_cppt(data: &[u8]) -> Result<SCppEntity> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2016, "CPPT")
		.context("Creating a ResourceConverter for CPPT resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given CPPT data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SCppEntity")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib DSWB")]
pub fn h2016_convert_dswb(data: &[u8]) -> Result<SwitchGroup> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2016, "DSWB")
		.context("Creating a ResourceConverter for DSWB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given DSWB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SwitchGroup")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib ECPB")]
pub fn h2016_convert_ecpb(data: &[u8]) -> Result<SExtendedCppEntityBlueprint> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2016, "ECPB")
		.context("Creating a ResourceConverter for ECPB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given ECPB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SExtendedCppEntityBlueprint")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib WSGB")]
pub fn h2016_convert_wsgb(data: &[u8]) -> Result<SwitchGroup> {
	let converter = ResourceConverter::new(resourcelib_ffi::WoaVersion::HM2016, "WSGB")
		.context("Creating a ResourceConverter for WSGB resources")?;
	let res = serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given WSGB data")?
			.as_str()
	)
	.context("Couldn't deserialize returned JsonString as SwitchGroup")?;
	res
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib format")]
pub fn convert_generic<T: DeserializeOwned>(data: &[u8], game: GameVersion, resource_type: ResourceType) -> Result<T> {
	let resource_extension = resource_type.as_ref();
	let game_version = match game {
		GameVersion::H1 => resourcelib_ffi::WoaVersion::HM2016,
		GameVersion::H2 => resourcelib_ffi::WoaVersion::HM2,
		GameVersion::H3 => resourcelib_ffi::WoaVersion::HM3
	};
	let converter = ResourceConverter::new(game_version, resource_extension).context(format!(
		"Creating a ResourceConverter for {} resources",
		resource_extension
	))?;
	serde_json::from_str(
		converter
			.memory_to_json_string(data)
			.context("Couldn't generate json string with the given UICB data")?
			.as_str()
	)
	.context(format!(
		"Couldn't deserialize returned JsonString as {}",
		std::any::type_name::<T>()
	))?
}

#[try_fn]
#[context("Couldn't convert binary data to ResourceLib format")]
pub fn convert_generic_str(data: &[u8], game: GameVersion, resource_type: ResourceType) -> Result<String> {
	let resource_extension = resource_type.as_ref();
	let game_version = match game {
		GameVersion::H1 => resourcelib_ffi::WoaVersion::HM2016,
		GameVersion::H2 => resourcelib_ffi::WoaVersion::HM2,
		GameVersion::H3 => resourcelib_ffi::WoaVersion::HM3
	};
	let converter = ResourceConverter::new(game_version, resource_extension).context(format!(
		"Creating a ResourceConverter for {} resources",
		resource_extension
	))?;

	converter
		.memory_to_json_string(data)
		.context("Couldn't generate json string with the given UICB data")?
}
