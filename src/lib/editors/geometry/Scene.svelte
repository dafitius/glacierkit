<script lang="ts">
	import { random } from "lodash"
	import { Edges, Gizmo, Grid, interactivity, OrbitControls, Outlines, useCursor } from "@threlte/extras"
	import { Canvas, T } from "@threlte/core"
	import { type GeometryEditorEntryContext, GeometryRenderMode } from "$lib/editors/geometry/GeometryTypes"
	import { BufferAttribute, BufferGeometry, Euler, MathUtils } from "three"
	import type { GeometryEditorEntryMesh } from "$lib/bindings-types"
	import Mesh from "$lib/editors/geometry/Mesh.svelte"

	export let objects: GeometryEditorEntryContext[] = []
	export let selectedObject: string | null = null

	export let showGrid = true
	export let showWireframe = false
	export let lodMask;

	interactivity()
</script>

<T.PerspectiveCamera
	makeDefault
	position={[1, 1, 1]}>
	<T.DirectionalLight position={[5, 5, 5]} />
	<OrbitControls enableDamping />
</T.PerspectiveCamera>

<T.AmbientLight color={0xaaaaaa} />
{#each objects as object}
	{#if object.data.meshes}
		{#each object.data.meshes as mesh}
			<Mesh
				visible={(lodMask & mesh.lodMask) === lodMask}
				on:click={(e) => {
					e.stopPropagation()
					selectedObject = object.id
				}}
				mesh={mesh}
				materials={object.data.materials}
				bind:showWireframe={showWireframe}
				bind:position={object.data.position}
				color1={object.renderMode === GeometryRenderMode.vertexColor}/>
		{/each}
	{/if}

{/each}
{#if showGrid}
	<Grid
		sectionColor={0xaaaaaa}
		sectionThickness={1}
		cellColor="#cccccc"
		gridSize={40}
	/>
{/if}