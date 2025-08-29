<script lang="ts">
	import { onDestroy, onMount } from "svelte"
	import jQuery from "jquery"
	import "jstree"
	import type { GeometryEditorEntryKind } from "$lib/bindings-types"
	import type { GeometryEditorEntryContext } from "$lib/editors/geometry/GeometryTypes"

	export let objects: GeometryEditorEntryContext[] = [];
	export let lodMask: number;

	export let selectedObject: string | null = null;

	let prevLodMask: number;

	$: if (lodMask !== prevLodMask) {
		prevLodMask = lodMask;
		refreshTree();
	}

	let treeDiv: HTMLDivElement;

	function getIconForKind(editorEntryKindElement: GeometryEditorEntryKind) {
		switch (editorEntryKindElement){
			case "mesh":
				return "fa fa-circle-nodes"
			case "weighted":
				return "fa fa-bone"
			case "linked":
				return "fa fa-link"
		}
	}

	// Helper to convert EditorEntry[] to jsTree JSON format
	function entriesToJsTreeData(entries: GeometryEditorEntryContext[]) {
		return entries.map((entry, idx) => {
			const children = [
				{
					id: entry.id,
					text: "Submeshes",
					children: entry.data.meshes.map((mesh, mIdx) => ({
						id: `obj_${idx}_mesh_${mIdx}`,
						text: `submesh-${mIdx} [${mesh.lodMask}]`,
						icon: (lodMask & mesh.lodMask) === lodMask ? "fa fa-circle" : "fa fa-circle-xmark",
					})),
					icon: "fa fa-cubes",
				}
			];

			// if (entry.data.colliders.length > 0) {
			// 	children.push({
			// 		id: `obj_${idx}_colliders`,
			// 		text: "Colliders",
			// 		children: [],
			// 		icon: "fa fa-person-falling-burst",
			// 	})
			// }

			if (entry.data.materials.length > 0) {
					children.push({
						id: `obj_${idx}_materials`,
						text: "Materials",
						children: entry.data.materials.map((material, mIdx) => ({
							id: `obj_${idx}_material_${mIdx}`,
							text: material.name,
							icon: "fa fa-circle-notch",
						})),
						icon: "fa fa-circle-notch",
					})
				}

			if (entry.data.rig !== null) {
				children.push({
					id: `obj_${idx}_rig`,
					text: `Rig`,
					children: [],
					icon: "fa fa-person",
				});
			}

			return {
				id: `obj_${idx}`,
				text: entry.data.name,
				children,
				icon: getIconForKind(entry.data.kind),
			};
		});
	}

	let treeInstance: any;

	function refreshTree() {
		if (treeInstance) {
			jQuery(treeDiv).jstree("destroy");
		}
		jQuery(treeDiv).jstree({
			core: {
				data: entriesToJsTreeData(objects),
				themes: {
					stripes: true,
				},
			},
			plugins: [],
		});
		treeInstance = jQuery(treeDiv);

		jQuery(treeDiv).on("select_node.jstree", function (e, data) {
			const node = data.node;
			const parents = node.parents.filter((p: string) => p !== "#");
			selectedObject = parents.length > 0 ? parents[parents.length - 1] : node.id;
		});
	}

	$: if (selectedObject && treeInstance) {
		// Find the index of the object with the selected name
		const idx = objects.findIndex(obj => obj.id === selectedObject);
		if (idx !== -1) {
			const nodeId = `obj_${idx}`;
			// Only select if not already selected
			const selected = treeInstance.jstree("get_selected");
			if (!selected.includes(nodeId)) {
				treeInstance.jstree("deselect_all");
				treeInstance.jstree("select_node", nodeId);
			}
		}
	}


	onMount(() => {
		refreshTree();
	});

	$: if (objects) {
		refreshTree();
	}

	onDestroy(() => {
		if (treeInstance) {
			jQuery(treeDiv).jstree("destroy");
			treeInstance = null;
		}
	});
</script>

<div>
	<div bind:this={treeDiv}></div>
</div>
