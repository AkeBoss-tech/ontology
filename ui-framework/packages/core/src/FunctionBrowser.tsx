import React, { useState } from 'react';
import { useQuery, useMutation, gql } from '@apollo/client';
import { useOntology } from './OntologyProvider';
import { PropertyEditor, PropertyDefinition } from './PropertyEditor';

const GET_FUNCTIONS = gql`
  query GetFunctions {
    getFunctions {
      id
      displayName
      description
      parameters {
        id
        displayName
        type
        required
      }
      returnType
      cacheable
    }
  }
`;

const EXECUTE_FUNCTION = gql`
  mutation ExecuteFunction($functionId: String!, $parameters: JSON!) {
    executeFunction(functionId: $functionId, parameters: $parameters) {
      value
      cached
    }
  }
`;

export interface FunctionBrowserProps {
  onResult?: (result: any) => void;
}

export function FunctionBrowser({ onResult }: FunctionBrowserProps) {
  const { client } = useOntology();
  const [selectedFunctionId, setSelectedFunctionId] = useState<string>();
  const [parameters, setParameters] = useState<Record<string, any>>({});
  const [executionResult, setExecutionResult] = useState<any>(null);

  const { data, loading, error } = useQuery(GET_FUNCTIONS, {
    client,
  });

  const [executeFunction, { loading: executing }] = useMutation(EXECUTE_FUNCTION, {
    client,
    onCompleted: (data) => {
      const result = JSON.parse(data.executeFunction.value);
      setExecutionResult(result);
      onResult?.(result);
    },
  });

  const functions = data?.getFunctions || [];
  const selectedFunction = functions.find((f: any) => f.id === selectedFunctionId);

  const handleExecute = async () => {
    if (!selectedFunctionId) return;

    // Convert parameters to JSON strings
    const params: Record<string, string> = {};
    for (const [key, value] of Object.entries(parameters)) {
      params[key] = JSON.stringify(value);
    }

    try {
      await executeFunction({
        variables: {
          functionId: selectedFunctionId,
          parameters: params,
        },
      });
    } catch (err) {
      console.error('Function execution error:', err);
    }
  };

  // Convert function parameters to PropertyDefinition format
  const parameterDefinitions: PropertyDefinition[] = selectedFunction
    ? selectedFunction.parameters.map((p: any) => ({
        id: p.id,
        displayName: p.displayName || p.id,
        type: p.type.toLowerCase().replace('propertytype::', ''),
        required: p.required,
      }))
    : [];

  return (
    <div className="function-browser space-y-6">
      <div>
        <h2 className="text-xl font-bold mb-4">Available Functions</h2>
        {loading ? (
          <div>Loading functions...</div>
        ) : error ? (
          <div className="text-red-500">Error: {error.message}</div>
        ) : functions.length === 0 ? (
          <div className="text-gray-500">No functions available</div>
        ) : (
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            {functions.map((func: any) => (
              <div
                key={func.id}
                onClick={() => {
                  setSelectedFunctionId(func.id);
                  setParameters({});
                  setExecutionResult(null);
                }}
                className={`p-4 border-2 rounded-lg cursor-pointer hover:bg-gray-50 transition-colors ${
                  selectedFunctionId === func.id
                    ? 'border-blue-500 bg-blue-50'
                    : 'border-gray-200'
                }`}
              >
                <div className="font-semibold">{func.displayName}</div>
                <div className="text-sm text-gray-500 mt-1">{func.id}</div>
                {func.description && (
                  <div className="text-sm text-gray-600 mt-2">{func.description}</div>
                )}
                <div className="text-xs text-gray-400 mt-2">
                  Returns: {func.returnType} {func.cacheable && '(cacheable)'}
                </div>
              </div>
            ))}
          </div>
        )}
      </div>

      {selectedFunction && (
        <div className="bg-white p-6 rounded-lg shadow">
          <h3 className="text-lg font-semibold mb-4">
            Execute: {selectedFunction.displayName}
          </h3>

          {parameterDefinitions.length > 0 ? (
            <div className="mb-4">
              <h4 className="font-medium mb-2">Parameters</h4>
              <PropertyEditor
                properties={parameterDefinitions}
                values={parameters}
                onChange={setParameters}
              />
            </div>
          ) : (
            <div className="text-gray-500 mb-4">No parameters required</div>
          )}

          <button
            onClick={handleExecute}
            disabled={executing}
            className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 disabled:bg-gray-400"
          >
            {executing ? 'Executing...' : 'Execute Function'}
          </button>

          {executionResult !== null && (
            <div className="mt-4 p-4 bg-gray-50 rounded border">
              <h4 className="font-medium mb-2">Result</h4>
              <pre className="text-sm overflow-auto">
                {JSON.stringify(executionResult, null, 2)}
              </pre>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
