<script lang="ts">
	import type { GeometryEditorEntry, GeometryEditorRequest } from "$lib/bindings-types"
	import { help } from "$lib/helpray"
	import { Gizmo, Grid, interactivity, OrbitControls } from "@threlte/extras"
	import { Pane, Splitpanes } from "svelte-splitpanes"
	import { rand } from "three/src/nodes/math/MathNode"
	import { random } from "lodash"
	import { onDestroy, onMount } from "svelte"
	import Tree from "$lib/editors/geometry/Tree.svelte"
	import {
		Checkbox,
		ContentSwitcher,
		FluidForm, FormGroup,
		Slider,
		Switch,
		Tab,
		TabContent,
		Tabs,
		Toggle
	} from "carbon-components-svelte"
	import { Cube, Globe } from "carbon-icons-svelte"
	import Scene from "$lib/editors/geometry/Scene.svelte"
	import { Canvas } from "@threlte/core"
	import { type GeometryEditorEntryContext, GeometryRenderMode } from "$lib/editors/geometry/GeometryTypes"

	export let id: string

	let selectedObject: string | null = null

	$: activeObject = objects.find((value) => {value.id === selectedObject});
	// function activeObject() {
	// 	return objects.find((value) => {value.id === selectedObject})
	// }

	//global settings
	let showGrid = true
	let showWireframe = false
	let lodMask = 0

	let objects: GeometryEditorEntryContext[] = [
		{
			id: "OBJ1",
			data: {
				name: "Object 1",
				kind: "mesh",
				position: [1, 1, 1],
				meshes: [
					{
						lodMask: 2,
						material: null
					},
					{
						lodMask: 4,
						material: null
					},
					{
						lodMask: 5,
						material: null
					}
				],
				rig: {},
				colliders: []
			},
			renderMode: GeometryRenderMode.material
		},
		{
			id: "OBJ2",
			data: {
				name: "Object 2",
				kind: "linked",
				position: [1, 1, 1],
				meshes: [
					{
						lodMask: 2,
						material: null
					}
				],
				rig: {},
				colliders: []
			},
			renderMode: GeometryRenderMode.material
		},
		{
			id: "OBJ3",
			data: {
				name: "Object 3",
				kind: "weighted",
				position: [2, 2, 2],
				meshes: [
					{
						lodMask: 2,
						material: null
					}
				],
				rig: {},
				colliders: []
			},
			renderMode: GeometryRenderMode.material
		}
	]

	export async function handleRequest(request: GeometryEditorRequest) {
		console.log(`Geometry editor ${id} handling request`, request)

		switch (request.type) {

			default:
				request satisfies never
				break
		}
	}

</script>

<div class="w-full h-full" use:help={{ title: "Geometry editor", description: "" }}>
	<Splitpanes theme="">
		<Pane maxSize={40}>
			<Splitpanes horizontal theme="">
				<Pane>
					<Tree objects={objects} bind:lodMask={lodMask} bind:selectedObject={selectedObject} />
				</Pane>
				<Pane>
					<Tabs>
						<Tab label="Global">
							<div class="flex items-center">
								<Globe />
								<p class="pl-1">Global</p>
							</div>
						</Tab>
						<Tab label="Object">
							<div class="flex items-center">
								<Cube />
								<p class="pl-1">Object</p>
							</div>
						</Tab>
						<svelte:fragment slot="content">
							<TabContent>
								<FormGroup legendText="Viewport options">
									<Checkbox id="show-grid-lines" labelText="Show grid" checked={showGrid}
											  on:change={() => {showGrid = !showGrid}} />
									<Checkbox id="show-wireframe" labelText="Show wireframe" checked={showWireframe}
											  on:change={() => {showWireframe = !showWireframe}} />
								</FormGroup>
								<FormGroup legendText="Glacier options">
									<Slider
										hideLabel
										labelText="Level of detail"
										hideTextInput
										minLabel="low"
										maxLabel="high"
										min={0}
										max={7}
										bind:value={lodMask}
										step={1}
									/>
								</FormGroup>
							</TabContent>
							<TabContent>
								<FormGroup legendText="Render options">
									<div>{activeObject?.renderMode.toString()}</div>
								</FormGroup>
							</TabContent>
						</svelte:fragment>
					</Tabs>
				</Pane>
			</Splitpanes>
		</Pane>

		<Pane>
			<Canvas>
				<Gizmo
					turnRate={0.5}
					verticalPlacement="top"
					horizontalPlacement="right"
					size={75}
				/>
				<Scene bind:objects={objects} bind:showGrid={showGrid} bind:showWireframe={showWireframe}
					   bind:selectedObject={selectedObject} />
			</Canvas>
		</Pane>
	</Splitpanes>
</div>

<style>
    :global(.splitpanes__pane.overflow-visible) {
        overflow: visible;
    }
</style>
