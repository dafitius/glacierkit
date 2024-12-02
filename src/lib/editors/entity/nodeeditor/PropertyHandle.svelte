<script context="module" lang="ts">
    import type { HandleProps } from '@xyflow/svelte';
  
    // Define the possible shapes
    export type HandleShape = 'circle' | 'square';
  
    // Extend HandleProps to include color and shape
    export interface CustomHandleProps extends HandleProps {
      color?: string;
      shape?: HandleShape;
    }
  </script>
  
  <script lang="ts">
    import { Handle, Position } from '@xyflow/svelte';
  
    // Define the props with appropriate types
    export let id: HandleProps['id'];
    export let type: HandleProps['type'];
    export let position: HandleProps['position'];
    export let style: string = '';
    let isConnectable: HandleProps['isConnectable'] = true;
  
    // New custom props
    export let color: string = 'red'; // Default color
    export let shape: HandleShape = 'circle'; // Default shape
  
    // Compute additional styles based on shape
    $: borderRadius = shape === 'circle' ? '50%' : '0%';
    $: combinedStyle = `
      width: 5px;
      height: 5px;
      background: ${color};
      border-radius: ${borderRadius};
      border-color: #00000000;
      ${style}
    `;
  </script>
  
  <Handle
    {id}
    {type}
    {position}
    style={combinedStyle}
    {isConnectable}
  />