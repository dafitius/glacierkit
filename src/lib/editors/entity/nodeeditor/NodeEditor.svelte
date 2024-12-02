<script lang="ts">
	import { writable } from "svelte/store"
	import ELK from "elkjs/lib/elk.bundled.js"
	import {
		SvelteFlow,
		Background,
		Position,
		ConnectionLineType,
		Panel,
		useSvelteFlow,
		type Node,
		type Edge
	} from "@xyflow/svelte"

	import "@xyflow/svelte/dist/style.css"

	import { onMount } from "svelte"
	import EntityNode from "./EntityNode.svelte"
	import type { EntityNodeData } from "$lib/bindings-types"

	const nodes = writable<Node[]>([])
	const edges = writable<Edge[]>([])
	const current_nodes = writable<Node[]>([])

	const nodeTypes = {
		entity: EntityNode
	}
	const { fitView } = useSvelteFlow()

	const elk = new ELK()

	var elk_algo = "layered"

	// Elk has a *huge* amount of options to configure. To see everything you can
	// tweak check out:
	//
	// - https://www.eclipse.org/elk/reference/algorithms.html
	// - https://www.eclipse.org/elk/reference/options.html
	var elkOptions = {
		"elk.algorithm": elk_algo,
		"elk.layered.spacing.nodeNodeBetweenLayers": "100",
		"elk.spacing.nodeNode": "80"
	}

	function getLayoutedElements(nodes: Node[], edges: Edge[], options = {}) {
		const isHorizontal = options?.["elk.direction"] === "RIGHT"
		const graph = {
			id: "root",
			layoutOptions: options,
			children: nodes.map((node) => ({
				...node,
				// Adjust the target and source handle positions based on the layout
				// direction.
				targetPosition: isHorizontal ? Position.Left : Position.Top,
				sourcePosition: isHorizontal ? Position.Right : Position.Bottom,

				// Hardcode a width and height for elk to use when layouting.
				width: 150,
				height: 50
			})),
			edges: edges
		}

		return elk
			.layout(graph)
			.then((layoutedGraph) => ({
				nodes: layoutedGraph.children.map((node) => ({
					...node,
					// React Flow expects a position property on the node instead of `x`
					// and `y` fields.
					position: { x: node.x, y: node.y }
				})),

				edges: layoutedGraph.edges
			}))
			.catch(console.error)
	}

	function onLayout(direction: string) {
		const opts = { "elk.direction": direction, ...elkOptions }
		const ns = $current_nodes
		const es = $edges

		getLayoutedElements(ns, es, opts).then(({ nodes: layoutedNodes, edges: layoutedEdges }) => {
			$nodes = layoutedNodes
			$edges = layoutedEdges

			fitView()

			window.requestAnimationFrame(() => fitView())
		})
	}

	onMount(() => {
		onLayout("DOWN")
	})

	export function clear() {
		$nodes = []
		$current_nodes = []
		$edges = []
		$nodes = $nodes
		$edges = $edges
		$current_nodes = $current_nodes
	}

	export function doLayout() {
		onLayout("DOWN")
	}

	export function addNode(node_data: EntityNodeData) {
		const newNode: Node = {
			id: node_data.id,
			type: "entity",
			data: {
				name: node_data.name,
				type: node_data.entityType,
				input_pins: node_data.inputPins,
				output_pins: node_data.outputPins,
				reference_pins: node_data.referencePins,
				exposed_entities: node_data.exposedEntities
			},
			position: { x: 0, y: 0 }
		}
		$current_nodes.push(newNode)
		// $edges.push({
		//   source: sourceNodeId,
		//   target: id,
		//   id: `${sourceNodeId}--${id}`
		// });

		$nodes = $nodes
		$edges = $edges
	}

</script>

<div style="height:100vh;">
	<SvelteFlow
		{nodes}
		{edges}
		{nodeTypes}
		colorMode={"dark"}
		fitView
		connectionLineType={ConnectionLineType.SmoothStep}
		defaultEdgeOptions={{ type: 'smoothstep', style: 'stroke: #94aecf;', animated: true }}
	>
		<Panel position="top-right">
			<button on:click={() => onLayout('DOWN')}>vertical layout</button>
			<button on:click={() => onLayout('RIGHT')}>horizontal layout</button>
			<input bind:value={elk_algo} on:change={() => onLayout('RIGHT')}>
		</Panel>

		<Background />
	</SvelteFlow>
</div>