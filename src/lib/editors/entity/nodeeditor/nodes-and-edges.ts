import type { Node, Edge } from '@xyflow/svelte';

const position = { x: 0, y: 0 };
const edgeType = 'smoothstep';

export const initialNodes: Node[] = [

  {
    id: '6f33c498bf310b25',
    type: 'entity',
    data: { 
      name: 'Spatial',
      type: 'COMPOSITE',
      inputs: [
        "Do",
      ],
      outputs: [
        "Done",
        "Started"
      ],
      properties: [
        {
          name: "m_aValues",
          type: "ref_array",
        },
        {
          name: "m_aValues",
          type: "ref",
        }
      ],
      exposedEntities: [
        {
          name: "[this]",
          in_use: false,
        },
        {
          name: "other",
          type: true,
        },
        {
          name: "other2",
          type: true,
        }
      ]
    },
    position: { x: 0, y: 0 },
  },
  {
    id: '7643c498bf310b25',
    type: 'entity',
    data: { 
      name: 'Entity',
      type: 'ENTITY',
      inputs: [],
      outputs: [],
      properties: [
        {
          name: "m_aValues",
          type: "ref",
          in_use: true

        },
        {
          name: "m_aValues2",
          type: "ref",
          in_use: true

        }
      ],
      exposedEntities: [
        {
          name: "[this]",
          in_use: false,
        }
      ]
    },
    position: { x: 0, y: 0 },
  },
  {
    id: '8633c495bf310b25',
    type: 'entity',
    data: { 
      name: 'Node',
      type: 'LOGIC',
      inputs: ["Input"],
      outputs: [],
      properties: [
        {
          name: "m_aValues",
          type: "ref",
          in_use: false

        }
      ],
      exposedEntities: [
        {
          name: "[this]",
          in_use: false,
        }
      ]
    },
    position: { x: 0, y: 0 },
  },
  ];

  export const initialEdges: Edge[] = [
    // {id: 'e12', source: '70d14c34fd6420ee', sourceHandle: 'OnTrue', target: 'e9e5bcda7e96630a', targetHandle: 'Reset', type: edgeType, animated: true}
    // { id: 'e1-2', source: '1', target: '2', type: edgeType, animated: true },
    // { id: 'e2-3', source: '2', target: '3', type: edgeType, animated: true },
    // { id: 'e3-4', source: '3', target: '4', type: edgeType, animated: true },
  ];