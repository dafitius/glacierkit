<script lang="ts">
	import { random } from "lodash"
	import { Edges, Gizmo, Grid, interactivity, OrbitControls, Outlines, useCursor } from "@threlte/extras"
	import { Canvas, T } from "@threlte/core"
	import type { GeometryEditorEntryContext } from "$lib/editors/geometry/GeometryTypes"

	export let objects: GeometryEditorEntryContext[] = []
	export let selectedObject: string | null = null

	export let showGrid = true
	export let showWireframe = false

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
	<T.Mesh
		receiveShadow
		castShadow
		position={object.data.position}
		on:click={(e) => {
				e.stopPropagation()
				selectedObject = object.id
			}}
	>
		<T.BoxGeometry args={[random(true), random(true), random(true)]} />
		<T.MeshStandardMaterial
			wireframe={showWireframe}
			fog
			transparent={object.id === selectedObject}
			opacity={object.id === selectedObject ? 1.0 : 0.5}

		/>
	</T.Mesh>


{/each}
{#if showGrid}
	<Grid
		sectionColor={0xaaaaaa}
		sectionThickness={1}
		cellColor="#cccccc"
		gridSize={40}
	/>
{/if}