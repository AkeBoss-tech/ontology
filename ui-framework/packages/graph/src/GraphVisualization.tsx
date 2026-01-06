import React from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

const TRAVERSE_GRAPH = gql`
  query TraverseGraph(
    $objectType: String!
    $objectId: String!
    $linkTypes: [String!]!
    $maxHops: Int!
    $aggregateProperty: String
    $aggregateOperation: String
  ) {
    traverseGraph(
      objectType: $objectType
      objectId: $objectId
      linkTypes: $linkTypes
      maxHops: $maxHops
      aggregateProperty: $aggregateProperty
      aggregateOperation: $aggregateOperation
    ) {
      objectIds
      aggregatedValue
      count
    }
  }
`;

export interface GraphVisualizationProps {
  objectType: string;
  objectId: string;
  linkTypes: string[];
  maxHops?: number;
  onNodeClick?: (objectId: string) => void;
}

export function GraphVisualization({
  objectType,
  objectId,
  linkTypes,
  maxHops = 3,
  onNodeClick,
}: GraphVisualizationProps) {
  const { client } = useOntology();

  const { data, loading, error } = useQuery(TRAVERSE_GRAPH, {
    client,
    variables: {
      objectType,
      objectId,
      linkTypes,
      maxHops,
    },
  });

  if (loading) return <div>Loading graph...</div>;
  if (error) return <div className="text-red-500">Error: {error.message}</div>;

  const result = data?.traverseGraph;
  if (!result) return <div>No graph data</div>;

  // Simple visualization - in production, use react-force-graph or vis-network
  return (
    <div className="graph-visualization border rounded p-4">
      <h3 className="text-lg font-semibold mb-4">Graph Traversal</h3>
      <div className="mb-2">
        <span className="font-medium">Nodes found: </span>
        <span>{result.count || result.objectIds.length}</span>
      </div>
      {result.aggregatedValue && (
        <div className="mb-2">
          <span className="font-medium">Aggregated value: </span>
          <span>{result.aggregatedValue}</span>
        </div>
      )}
      <div className="space-y-1 max-h-64 overflow-y-auto">
        {result.objectIds.map((id: string) => (
          <div
            key={id}
            onClick={() => onNodeClick?.(id)}
            className="p-2 border rounded cursor-pointer hover:bg-gray-50"
          >
            {id}
          </div>
        ))}
      </div>
    </div>
  );
}







