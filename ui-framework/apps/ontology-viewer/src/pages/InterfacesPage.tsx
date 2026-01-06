import React, { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

const GET_INTERFACES = gql`
  query GetInterfaces {
    getInterfaces {
      id
      displayName
      properties {
        id
        displayName
        type
        required
      }
      implementers {
        objectType
        count
      }
    }
  }
`;

export default function InterfacesPage() {
    const { client } = useOntology();
    const { data, loading, error } = useQuery(GET_INTERFACES, { client });
    const [selectedInterface, setSelectedInterface] = useState<string | null>(null);

    if (loading) return <div className="text-center py-12">Loading interfaces...</div>;
    if (error) return <div className="text-red-500 p-4">Error: {error.message}</div>;

    const interfaces = data?.getInterfaces || [];

    return (
        <div className="px-4 py-6 sm:px-0">
            <h1 className="text-2xl font-bold mb-6">Interfaces</h1>
            <p className="text-gray-600 mb-6">
                Interfaces define contracts that multiple object types can implement.
                They enable polymorphic queries across different types that share common properties.
            </p>

            {interfaces.length === 0 ? (
                <div className="bg-white rounded-lg shadow-sm border p-8 text-center">
                    <div className="text-4xl mb-4">📋</div>
                    <p className="text-gray-500 mb-2">No interfaces defined in this ontology</p>
                    <p className="text-sm text-gray-400">
                        Interfaces can be added to your ontology to enable polymorphic queries across object types.
                    </p>
                </div>
            ) : (
                <div className="space-y-4">
                    {interfaces.map((iface: any) => {
                        const totalCount = iface.implementers?.reduce((sum: number, impl: any) => sum + (impl.count || 0), 0) || 0;

                        return (
                            <div
                                key={iface.id}
                                className="bg-white rounded-lg shadow-sm border overflow-hidden"
                            >
                                <div
                                    onClick={() => setSelectedInterface(selectedInterface === iface.id ? null : iface.id)}
                                    className="p-4 cursor-pointer hover:bg-gray-50 flex items-center justify-between"
                                >
                                    <div className="flex items-center gap-4">
                                        <div className="w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center">
                                            <span className="text-purple-600 font-bold">I</span>
                                        </div>
                                        <div>
                                            <h3 className="font-semibold">{iface.displayName || iface.id}</h3>
                                            <p className="text-sm text-gray-500">
                                                {iface.properties?.length || 0} properties • {iface.implementers?.length || 0} implementers
                                            </p>
                                        </div>
                                    </div>
                                    <div className="text-right">
                                        <span className="text-2xl font-bold text-purple-600">{totalCount}</span>
                                        <p className="text-xs text-gray-500">total objects</p>
                                    </div>
                                </div>

                                {selectedInterface === iface.id && (
                                    <div className="p-4 bg-gray-50 border-t">
                                        {/* Properties */}
                                        <div className="mb-4">
                                            <h4 className="font-medium mb-2">Required Properties</h4>
                                            {iface.properties?.length > 0 ? (
                                                <div className="flex flex-wrap gap-2">
                                                    {iface.properties.map((prop: any) => (
                                                        <span
                                                            key={prop.id}
                                                            className="px-2 py-1 bg-white border rounded text-sm"
                                                        >
                                                            <span className="font-medium">{prop.displayName || prop.id}</span>
                                                            <span className="text-gray-400 ml-1">: {prop.type}</span>
                                                            {prop.required && <span className="text-red-500 ml-1">*</span>}
                                                        </span>
                                                    ))}
                                                </div>
                                            ) : (
                                                <p className="text-sm text-gray-500">No required properties</p>
                                            )}
                                        </div>

                                        {/* Implementers */}
                                        <div>
                                            <h4 className="font-medium mb-2">Implementing Object Types</h4>
                                            {iface.implementers?.length > 0 ? (
                                                <div className="flex flex-wrap gap-2">
                                                    {iface.implementers.map((impl: any) => (
                                                        <span
                                                            key={impl.objectType}
                                                            className="px-3 py-1 bg-purple-100 text-purple-800 rounded-full text-sm"
                                                        >
                                                            {impl.objectType} ({impl.count})
                                                        </span>
                                                    ))}
                                                </div>
                                            ) : (
                                                <p className="text-sm text-gray-500">No implementing types</p>
                                            )}
                                        </div>

                                        <div className="mt-4 pt-4 border-t">
                                            <p className="text-xs text-gray-400">
                                                Interface ID: <code className="bg-gray-200 px-1 rounded">{iface.id}</code>
                                            </p>
                                        </div>
                                    </div>
                                )}
                            </div>
                        );
                    })}
                </div>
            )}

            <div className="mt-8 bg-purple-50 rounded-lg p-6 border border-purple-100">
                <h2 className="font-semibold mb-2">💡 Tip: Query by Interface</h2>
                <p className="text-sm text-purple-800">
                    Use the <strong>Interface Explorer</strong> app to query objects that implement a specific interface.
                    This lets you work with objects across multiple types that share common properties.
                </p>
            </div>
        </div>
    );
}
