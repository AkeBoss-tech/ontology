import React, { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

const GET_OBJECT_TYPES = gql`
  query GetObjectTypes {
    getObjectTypes {
      id
      displayName
    }
  }
`;

// We'll need to make a separate call for properties of each type
// since the schema might not expose all details in one query

export default function ObjectTypesPage() {
    const { client } = useOntology();
    const { data, loading, error } = useQuery(GET_OBJECT_TYPES, { client });
    const [selectedType, setSelectedType] = useState<string | null>(null);

    if (loading) return <div className="text-center py-12">Loading object types...</div>;
    if (error) return <div className="text-red-500 p-4">Error: {error.message}</div>;

    const objectTypes = data?.getObjectTypes || [];

    return (
        <div className="px-4 py-6 sm:px-0">
            <h1 className="text-2xl font-bold mb-6">Object Types</h1>
            <p className="text-gray-600 mb-6">
                Object types define the entities in your domain. Each object type has properties that describe its attributes.
            </p>

            {objectTypes.length === 0 ? (
                <div className="bg-white rounded-lg shadow-sm border p-8 text-center">
                    <p className="text-gray-500">No object types defined in this ontology</p>
                </div>
            ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {objectTypes.map((ot: any) => (
                        <div
                            key={ot.id}
                            onClick={() => setSelectedType(selectedType === ot.id ? null : ot.id)}
                            className={`bg-white rounded-lg shadow-sm border p-4 cursor-pointer hover:shadow-md transition-shadow ${selectedType === ot.id ? 'ring-2 ring-blue-500' : ''
                                }`}
                        >
                            <div className="flex items-center gap-3">
                                <div className="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                                    <span className="text-blue-600 font-bold text-lg">
                                        {(ot.displayName || ot.id).charAt(0).toUpperCase()}
                                    </span>
                                </div>
                                <div>
                                    <h3 className="font-semibold">{ot.displayName || ot.id}</h3>
                                    <p className="text-sm text-gray-500 font-mono">{ot.id}</p>
                                </div>
                            </div>

                            {selectedType === ot.id && (
                                <div className="mt-4 pt-4 border-t">
                                    <p className="text-sm text-gray-600">
                                        Click "Object Types" in the Object Explorer to view and query objects of this type.
                                    </p>
                                    <div className="mt-2 text-xs text-gray-400">
                                        ID: <code className="bg-gray-100 px-1 rounded">{ot.id}</code>
                                    </div>
                                </div>
                            )}
                        </div>
                    ))}
                </div>
            )}

            <div className="mt-8 bg-gray-50 rounded-lg p-6 border">
                <h2 className="font-semibold mb-2">📊 Total Object Types: {objectTypes.length}</h2>
                <p className="text-sm text-gray-600">
                    Object types are the foundation of your ontology. They represent different kinds of entities
                    in your domain, such as customers, products, orders, or any other concept relevant to your application.
                </p>
            </div>
        </div>
    );
}
