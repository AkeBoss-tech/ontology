import React, { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

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

export default function FunctionsPage() {
    const { client } = useOntology();
    const { data, loading, error } = useQuery(GET_FUNCTIONS, { client });
    const [selectedFunction, setSelectedFunction] = useState<string | null>(null);

    if (loading) return <div className="text-center py-12">Loading functions...</div>;
    if (error) return <div className="text-red-500 p-4">Error: {error.message}</div>;

    const functions = data?.getFunctions || [];

    return (
        <div className="px-4 py-6 sm:px-0">
            <h1 className="text-2xl font-bold mb-6">Functions</h1>
            <p className="text-gray-600 mb-6">
                Functions are computed values defined in your ontology. They can aggregate data,
                traverse links, or perform calculations on object properties.
            </p>

            {functions.length === 0 ? (
                <div className="bg-white rounded-lg shadow-sm border p-8 text-center">
                    <div className="text-4xl mb-4">⚡</div>
                    <p className="text-gray-500 mb-2">No functions defined in this ontology</p>
                    <p className="text-sm text-gray-400">
                        Functions can be added to your ontology configuration to enable computed properties and aggregations.
                    </p>
                </div>
            ) : (
                <div className="space-y-4">
                    {functions.map((fn: any) => (
                        <div
                            key={fn.id}
                            className="bg-white rounded-lg shadow-sm border overflow-hidden"
                        >
                            <div
                                onClick={() => setSelectedFunction(selectedFunction === fn.id ? null : fn.id)}
                                className="p-4 cursor-pointer hover:bg-gray-50 flex items-center justify-between"
                            >
                                <div className="flex items-center gap-4">
                                    <div className="w-10 h-10 bg-orange-100 rounded-lg flex items-center justify-center">
                                        <span className="text-orange-600 font-bold">ƒ</span>
                                    </div>
                                    <div>
                                        <h3 className="font-semibold">{fn.displayName || fn.id}</h3>
                                        {fn.description && (
                                            <p className="text-sm text-gray-500">{fn.description}</p>
                                        )}
                                    </div>
                                </div>
                                <div className="flex items-center gap-2">
                                    {fn.cacheable && (
                                        <span className="px-2 py-1 bg-green-100 text-green-700 text-xs rounded">
                                            Cacheable
                                        </span>
                                    )}
                                    <span className="px-2 py-1 bg-gray-100 text-gray-600 text-xs rounded font-mono">
                                        → {fn.returnType}
                                    </span>
                                </div>
                            </div>

                            {selectedFunction === fn.id && (
                                <div className="p-4 bg-gray-50 border-t">
                                    <h4 className="font-medium mb-2">Parameters</h4>
                                    {fn.parameters?.length > 0 ? (
                                        <div className="space-y-2">
                                            {fn.parameters.map((param: any) => (
                                                <div key={param.id} className="flex items-center gap-2 text-sm">
                                                    <code className="bg-white px-2 py-1 rounded border">
                                                        {param.displayName || param.id}
                                                    </code>
                                                    <span className="text-gray-500">:</span>
                                                    <span className="text-blue-600 font-mono">{param.type}</span>
                                                    {param.required && (
                                                        <span className="text-red-500 text-xs">required</span>
                                                    )}
                                                </div>
                                            ))}
                                        </div>
                                    ) : (
                                        <p className="text-sm text-gray-500">No parameters</p>
                                    )}

                                    <div className="mt-4 pt-4 border-t">
                                        <p className="text-xs text-gray-400">
                                            Function ID: <code className="bg-gray-200 px-1 rounded">{fn.id}</code>
                                        </p>
                                    </div>
                                </div>
                            )}
                        </div>
                    ))}
                </div>
            )}

            <div className="mt-8 bg-orange-50 rounded-lg p-6 border border-orange-100">
                <h2 className="font-semibold mb-2">💡 Tip: Execute Functions</h2>
                <p className="text-sm text-orange-800">
                    Use the <strong>Function Executor</strong> app to run these functions with your own parameters
                    and see the results. Functions marked as "Cacheable" will return cached results for improved performance.
                </p>
            </div>
        </div>
    );
}
