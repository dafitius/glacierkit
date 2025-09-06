<script lang="ts">
	import { BufferGeometry, BufferAttribute, MathUtils, Color, MeshStandardMaterial, TextureLoader } from "three"
	import { T, useLoader } from "@threlte/core"
	import { useTexture } from "@threlte/extras"
	import { createEventDispatcher } from "svelte"
	import type { GeometryEditorEntryMaterial, GeometryEditorEntryMesh } from "$lib/bindings-types"
	import { convertFileSrc } from "@tauri-apps/api/core"

	export let mesh: GeometryEditorEntryMesh
	export let materials: GeometryEditorEntryMaterial[]
	export let position: number | [x: number, y: number, z: number] | undefined
	export let showWireframe: boolean
	export let color1: boolean

	export let visible: boolean = true

	export let onClick: (event: MouseEvent) => void = () => {
	}
	const dispatch = createEventDispatcher()

	function handleClick(event: MouseEvent) {
		onClick(event)
		dispatch("click", event)
	}


	// load a resource

	function get_texture_map(map_name: string) {

		let texture_info = materials[mesh.materialIndex].textures.find((texture) => {
			return texture.name === map_name
		})
		console.log("TEXTURE info:" + texture_info)
		if (texture_info) {
			let asset_path = convertFileSrc(texture_info.path, "blob");
			// console.log("Loading: ", asset_path)
			let textureLoader = new TextureLoader();
			textureLoader.setCrossOrigin("anonymous");
			textureLoader.setWithCredentials(false);
			textureLoader.setRequestHeader({
				"Origin": "",
				"Test-Value": "Hello!"
			})
			let t = textureLoader.load(asset_path, () => {}, () => {}, () => {
				console.log("TEXTURE: WHOMP WHOMP2")
			})

			return t;
			//
			// console.log("TEXTURE maybe -> ", t)
			//
			// const { load } = useLoader(TextureLoader,{
			// 	extend: (loader) => {
			// 		loader.setCrossOrigin("anonymous");
			// 		loader.setWithCredentials(false);
			// 		loader.setRequestHeader({
			// 			"Origin": "",
			// 			"Test-Value": "Hello!"
			// 		})
			// 		loader.load(asset_path, () => {}, () => {}, () => {})
			// 	}
			// });
			//
			//
			// return load(asset_path)
			// return useTexture(asset_path)

			// let texture_loader = new TextureLoader();
			// texture_loader.crossOrigin = "anonymous";
			// texture_loader.setCrossOrigin("anonymous");
			//
			// return texture_loader.load(asset_path, () => {}, () => {}, () => {
			// 	console.log("TEXTURE: WHOMP WHOMP")
			// });

		}
	}

	$: geometry = (() => {
		const geometry = new BufferGeometry()
		if (visible == false) {
			return geometry
		}

		geometry.setAttribute(
			"position",
			new BufferAttribute(
				new Float32Array(mesh.positionBuffer.flat()),
				3
			)
		)
		geometry.setAttribute(
			"normal",
			new BufferAttribute(
				new Float32Array(mesh.normalBuffer.flat()),
				3
			)
		)
		geometry.setAttribute(
			"uv",
			new BufferAttribute(
				new Float32Array(mesh.uvBuffer.flat()),
				2
			)
		)


		if (mesh.colorBuffer) {
			console.log(new Float32Array(mesh.colorBuffer.flat()))
			geometry.setAttribute(
				"color",
				new BufferAttribute(
					new Float32Array(mesh.colorBuffer.flat()),
					4
				)
			)
		}

		geometry.setIndex(mesh.indices)
		return geometry
	})()


	export function numToThreeColor(value: number): Color {
		const b = (value >> 16) & 0xff
		const g = (value >> 8) & 0xff
		const r = value & 0xff
		return new Color(r / 255, g / 255, b / 255)
	}

	$: material = (() => {
		const material = new MeshStandardMaterial()

		if (showWireframe) {
			material.wireframe = true
			material.color = numToThreeColor(mesh.wireColor)
			return material
		}

		if (color1) {
			if (mesh.colorBuffer) {
				material.vertexColors = true
				material.transparent = true
			} else {
				material.color = numToThreeColor(mesh.constColor)
			}
		}

		console.log("Materials:")
		console.log(materials)

		let albedo_map = get_texture_map("mapTexture2D_01")
		if (albedo_map) {
			// material.map = albedo_map
			albedo_map.then((texture) => {
				material.map = texture
			}).catch(() => {
				console.log("ALB TEXTURE FAILED TO LOAD")
			})
		}

		let normal_map = get_texture_map("mapTexture2DNormal_01")
		if (normal_map) {
			// material.normalMap = normal_map
			normal_map.then((texture) => {
				material.normalMap = texture
			}).catch(() => {
				console.log("NORM TEXTURE FAILED TO LOAD")
			})
		}

		return material
	})()

</script>

<T.Mesh
	receiveShadow
	castShadow
	position={position}
	rotation.x={-90 * MathUtils.DEG2RAD}
	on:click={handleClick}>
	<T is={geometry} />
	<T is={material} />
</T.Mesh>

