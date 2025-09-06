<script lang="ts">
	import type { GeometryEditorRequest } from "$lib/bindings-types"
	import { help } from "$lib/helpray"
	import { Gizmo } from "@threlte/extras"
	import { Pane, Splitpanes } from "svelte-splitpanes"
	import Tree from "$lib/editors/geometry/Tree.svelte"
	import { Checkbox, FormGroup, FormLabel, Slider, Tab, TabContent, Tabs } from "carbon-components-svelte"
	import { Cube, Globe } from "carbon-icons-svelte"
	import Scene from "$lib/editors/geometry/Scene.svelte"
	import { Canvas } from "@threlte/core"
	import { type GeometryEditorEntryContext, GeometryRenderMode } from "$lib/editors/geometry/GeometryTypes"
	import { onMount } from "svelte"
	import { convertFileSrc } from "@tauri-apps/api/core"

	export let id: string

	let selectedObject: string | null = null

	$: activeObject = objects.find((value) => {value.id === selectedObject});
	// function activeObject() {
	// 	return objects.find((value) => {value.id === selectedObject})
	// }

	//global settings
	let showGrid = true
	let showWireframe = false
	let lodSliderValue = 0;
	$: lodMask = (1 << (7 - lodSliderValue))

	let objects: GeometryEditorEntryContext[] = []

	onMount(async () => {

	})


	export async function handleRequest(request: GeometryEditorRequest) {
		// console.log(`Geometry editor ${id} handling request`, request)

		switch (request.type) {
			case "initialise":
				id = request.data.id
				break
			case "addObject":
				objects = [
					...objects,
					{
					id: request.data.obj_id,
					data: request.data.data,
					renderMode: GeometryRenderMode.material
					}];
				break


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
									<FormLabel>LoD {lodSliderValue + 1}</FormLabel>
									<Slider
										hideLabel
										labelText="Level of detail"
										hideTextInput
										minLabel="low"
										maxLabel="high"
										min={0}
										max={7}
										bind:value={lodSliderValue}
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
				<Scene bind:objects={objects} bind:lodMask={lodMask} bind:showGrid={showGrid} bind:showWireframe={showWireframe}
					   bind:selectedObject={selectedObject} />
			</Canvas>
		</Pane>
	</Splitpanes>
<!--	<img-->
<!--		alt=""-->
<!--		class="h-[30vh] bg-[#7f7f7f]"-->
<!--		style="image-rendering: pixelated"-->
<!--		src={convertFileSrc(objects[0].data.materials[0].textures[0].path)}-->
<!--	/>-->
</div>

<style>
    :global(.splitpanes__pane.overflow-visible) {
        overflow: visible;
    }
</style>
