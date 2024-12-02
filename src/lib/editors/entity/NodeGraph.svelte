<script lang="ts">
   import type { EntityNodeData, NodeEditorRequest } from "$lib/bindings-types"
    import { onMount } from "svelte"
    import { help } from "$lib/helpray"
    import {SvelteFlowProvider} from "@xyflow/svelte";
    import GraphEditor from "$lib/editors/entity/nodeeditor/NodeEditor.svelte";
    import { TextInput } from "carbon-components-svelte"
   import { event } from "$lib/utils"

    export let editorID: string
    export let entityPath: string = ""

    let graph : GraphEditor;

   async function change_entity(editor_id: string, entity_id: string){
      await event({
         type: "editor",
         data: {
            type: "entity",
            data: {
               type: "nodeEditor",
               data: {
                  type: "updateContent",
                  data: {
                     editor_id: editor_id,
                     entity_id: entity_id,
                  }
               }
            }
         }
      })
   }

    export async function handleRequest(request: NodeEditorRequest) {
        console.log(`Node graph editor for editor ${editorID} handling request`, request)

       function setNodes(nodes: EntityNodeData[]) {
           graph.clear();
          for (const node of nodes) {
             graph.addNode(node);
          }
          graph.doLayout();
       }

       switch (request.type) {
           case "changeEntity":
              await change_entity(request.data.editor_id, request.data.entity_id);
              break;
           case "replaceContent":
              entityPath = request.data.entity_path;
              setNodes(request.data.nodes);
              break;
            default:
                request satisfies never
                break
        }
    }

    onMount(async () => {

    })

</script>

<div class="h-full w-full overflow-y-auto" use:help={{ title: "Metadata", description: "This view lets you see and edit the metadata of an entity." }}>
   <TextInput readonly bind:value={entityPath}/>
   <SvelteFlowProvider>
      <GraphEditor bind:this={graph}/>
   </SvelteFlowProvider>
</div>
