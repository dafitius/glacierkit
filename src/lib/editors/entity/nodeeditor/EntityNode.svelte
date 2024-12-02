<script lang="ts">
  import { Handle, Position, type NodeProps } from "@xyflow/svelte";
  import PropertyHandle from "./PropertyHandle.svelte";
  import type { EntityNodeRefPin } from "$lib/bindings-types"

  type $$Props = NodeProps;

  export let data: $$Props["data"];
  export let isConnectable: $$Props["isConnectable"];

  const { name, type } = data;
  const inputs = data.input_pins as string[];
  const outputs = data.output_pins as string[];
  const referencePins = data.reference_pins as EntityNodeRefPin[];
  const exposedEntities = data.exposed_entities as string[];
</script>

<div class="node">

{#if referencePins.length > 0}
  <div>
    <div class="entityrefs">
      {#each referencePins as ref, i}
        <div class="entityref-container" style="left: {8 * (i + 1)}%">
          {#if (ref.kind === "refArray")}
            <PropertyHandle type="source" position={Position.Top} shape="square" id={ref.name} />
          {:else}
            <PropertyHandle type="source" position={Position.Top} id={ref.name} />
          {/if}
          <div class="handle-line"></div>
        </div>
      {/each}
    </div>
  </div>
{/if}

  <!-- Title -->
  <div class="header-container">
    <div class = "header-wrapper">
      <div class="title">{name}</div>
      <div class="subtitle">{type}</div>
    </div>
  </div>

  <div style="padding-top: 5px;"></div>

  <!-- Inputs and Outputs -->
  {#if inputs.length > 0 || outputs.length > 0}
    <div class="pins">
      <!-- Inputs -->
      <div class="inputs">
        {#each inputs as input}
          <div class="input-container">
            <div class="pin-handle">
              <Handle type="target" position={Position.Left} id={input} style="background: #a1a291;" />
            </div>
            <div class="label">{input}</div>
          </div>
        {/each}
      </div>

      <!-- Outputs -->
      <div class="outputs">
        {#each outputs || [] as output}
          <div class="output-container">
            <div class="label">{output}</div>

            <div class="pin-handle">
              <Handle type="source" position={Position.Right} id={output} style="background: #a1a291;"/>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {/if}


  {#if exposedEntities.length > 0}
  <div>
    <div class="entityrefs">
      {#each exposedEntities as entity, i}
        <div class="exposed-entity-container" style="left: {8 * (i + 1)}%">
          <div class="handle-line"></div>
          <PropertyHandle type="target" position={Position.Bottom} id={entity} />
        </div>
      {/each}
    </div>
  </div>
{/if}
</div>

<style>
  .node {
    background: #232227;
    border: 1px solid #00000000;
    border-radius: 5px;
    display: flex;
    flex-direction: column;
    align-items: center;
    color: #ffffff;
    font-family: Arial, sans-serif;
    box-shadow: 0px 4px 6px rgba(0, 0, 0, 0.1);
    width: fit-content;
    min-width: 10vw;
    overflow: hidden;
  }

  .header-wrapper {
      overflow: hidden;             /* Clips the header-container's rounded corners */
  }

  .header-container {
  background-color: #3190bc; /* Green-ish background */
  text-align: left;          /* Align text to the left */
  padding-top: 5px;             /* Optional: Adds some padding */
  padding-bottom: 5px;             /* Optional: Adds some padding */
  padding-left: 5px;             /* Optional: Adds some padding */
  width: 100%;
  box-sizing: border-box;     /* Ensures padding doesn't increase the width */
  overflow: hidden;
}


.subtitle {
  padding-top: 2px;
  font-size: 8px;
}

  .pins {
    display: flex;
    width: 100%;
    justify-content: space-between;
    padding-bottom: 5%;
  }

  .inputs,
  .outputs {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .label {
    font-size: 7px;
    white-space: nowrap;
  }

  .input-container,
  .output-container {
    display: flex;
    align-items: center;
    padding-top: 10%;
  }

  .input-container {
    justify-content: flex-start;
  }

  .output-container {
    justify-content: flex-end;
  }

  .pin-handle {
    width: 5px;
    height: 5px;
    position: relative;
  }

  .entityref-container {
    position: absolute;
    display: flex;
    flex-direction: column;
    align-items: center;
    transform: translateX(-50%); /* Center the handle horizontally */
  }

  .entityrefs {
    position: absolute;
    left: 0;
    width: 100%;
    display: flex;
    justify-content: flex-start;
    align-items: flex-end;
    padding: 0 10px;
    box-sizing: border-box;
  }

  .exposed-entity-container {
        position: absolute;
    display: flex;
    flex-direction: column;
    align-items: center;
    transform: translateX(-50%); /* Center the handle horizontally */
  }

  .handle-line {
    width: 1px;
    height: 5px;
    background-color: rgb(20, 20, 20);
    margin-top: 2px;
  }

</style>
