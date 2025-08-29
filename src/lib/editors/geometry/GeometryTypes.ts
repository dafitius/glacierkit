import type { GeometryEditorObject } from "$lib/bindings-types"

export enum GeometryRenderMode {
	material,
	vertexColor,
	boneWeights
}

export type GeometryEditorEntryContext = {
	id: string
	data: GeometryEditorObject
	renderMode: GeometryRenderMode
}
