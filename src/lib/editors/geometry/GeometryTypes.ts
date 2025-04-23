import type { GeometryEditorEntry } from "$lib/bindings-types"

export enum GeometryRenderMode {
	material,
	vertexColor,
	boneWeights
}

export type GeometryEditorEntryContext = {
	id: string
	data: GeometryEditorEntry
	renderMode: GeometryRenderMode
}
