<script lang="ts">
	import type ModelViewerRequest from "$lib/bindings-types"
	import { event } from "$lib/utils"
	import { onMount } from "svelte"
	import { help } from "$lib/helpray"

	export let id: string

	export async function handleRequest(request: ModelViewerRequest) {
		console.log(`Repository patch editor ${id} handling request`, request)

		switch (request.type) {
			case "replaceGeometry":
				break

			default:
				request satisfies never
				break
		}
	}

	onMount(async () => {
		await event({
			type: "editor",
			data: {
				type: "modelViewer",
				data: {
					type: "initialise",
					data: {
						id
					}
				}
			}
		})
	})
</script>

<div class="grid grid-cols-4 gap-4 w-full h-full">
	<div class="h-full">
		<div class="h-1/3 flex flex-col" use:help={{ title: "Modified repository items", description: "Items that have been added or modified from their original state by your edits." }}>
            Information here please -o-
		</div>
	</div>
</div>
