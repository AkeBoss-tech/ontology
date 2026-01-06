import React, { useState } from 'react';
import { useQuery, useMutation } from '@apollo/client';
import { GET_ONTOLOGY_STRUCTURE } from '../graphql/ontology-queries';
import { BIND_MODEL, UNBIND_MODEL, GET_MODEL_BINDINGS } from '../graphql/model-queries';
import { LinkIcon, TrashIcon } from '@heroicons/react/24/outline';

interface ModelBindingProps {
    modelId: string;
}

export default function ModelBinding({ modelId }: ModelBindingProps) {
    const [selectedObjectType, setSelectedObjectType] = useState<string>('');
    const [selectedProperty, setSelectedProperty] = useState<string>('');

    // key to force refetch after mutation
    const [key, setKey] = useState(0);

    const { loading: ontologyLoading, data: ontologyData } = useQuery(GET_ONTOLOGY_STRUCTURE);
    const { loading: bindingsLoading, data: bindingsData, refetch } = useQuery(GET_MODEL_BINDINGS, {
        pollInterval: 5000,
    });

    const [bindModel] = useMutation(BIND_MODEL, {
        onCompleted: () => {
            refetch();
            setSelectedObjectType('');
            setSelectedProperty('');
        }
    });

    const [unbindModel] = useMutation(UNBIND_MODEL, {
        onCompleted: () => refetch()
    });

    if (ontologyLoading || bindingsLoading) return <div className="p-4 text-center text-gray-500">Loading binding options...</div>;

    const objectTypes = ontologyData?.ontology?.objectTypes || [];
    const allBindings = bindingsData?.modelBindings || [];

    // Filter bindings for THIS model
    const myBindings = allBindings.filter((b: any) => b.modelId === modelId);

    const handleBind = () => {
        if (!selectedObjectType || !selectedProperty) return;

        bindModel({
            variables: {
                input: {
                    modelId: modelId,
                    objectType: selectedObjectType,
                    propertyId: selectedProperty,
                    inputProperties: [], // TODO: Allow mapping inputs
                }
            }
        });
    };

    const handleUnbind = (objectType: string, propertyId: string) => {
        if (confirm(`Are you sure you want to unbind this model from ${objectType}.${propertyId}?`)) {
            unbindModel({
                variables: {
                    objectType,
                    propertyId
                }
            });
        }
    };

    const selectedTypeObj = objectTypes.find((ot: any) => ot.id === selectedObjectType);

    return (
        <div className="bg-white shadow sm:rounded-lg border border-gray-200 mt-6">
            <div className="px-4 py-5 sm:px-6 border-b border-gray-200">
                <h3 className="text-lg leading-6 font-medium text-gray-900 flex items-center">
                    <LinkIcon className="h-5 w-5 mr-2 text-gray-500" />
                    Property Bindings
                </h3>
                <p className="mt-1 max-w-2xl text-sm text-gray-500">
                    Bind this model to object properties to drive their values automatically.
                </p>
            </div>

            <div className="px-4 py-5 sm:p-6">
                {/* Existing Bindings List */}
                {myBindings.length > 0 && (
                    <div className="mb-6">
                        <h4 className="text-sm font-medium text-gray-900 mb-3">Active Bindings</h4>
                        <ul className="divide-y divide-gray-200 border rounded-md">
                            {myBindings.map((binding: any) => (
                                <li key={`${binding.objectType}-${binding.propertyId}`} className="px-4 py-3 flex items-center justify-between text-sm">
                                    <div className="flex items-center">
                                        <span className="font-medium text-gray-900 mr-2">{binding.objectType}</span>
                                        <span className="text-gray-500 mx-2">→</span>
                                        <span className="font-mono text-gray-700 bg-gray-100 px-2 py-0.5 rounded">
                                            {binding.propertyId}
                                        </span>
                                    </div>
                                    <button
                                        onClick={() => handleUnbind(binding.objectType, binding.propertyId)}
                                        className="text-red-600 hover:text-red-800 p-1 rounded hover:bg-red-50"
                                        title="Remove Binding"
                                    >
                                        <TrashIcon className="h-4 w-4" />
                                    </button>
                                </li>
                            ))}
                        </ul>
                    </div>
                )}

                {/* New Binding Form */}
                <div className="bg-gray-50 p-4 rounded-md border border-gray-200">
                    <h4 className="text-sm font-medium text-gray-900 mb-3">Create New Binding</h4>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4 items-end">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Object Type</label>
                            <select
                                value={selectedObjectType}
                                onChange={(e) => {
                                    setSelectedObjectType(e.target.value);
                                    setSelectedProperty('');
                                }}
                                className="block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
                            >
                                <option value="">Select Object Type...</option>
                                {objectTypes.map((ot: any) => (
                                    <option key={ot.id} value={ot.id}>{ot.id}</option>
                                ))}
                            </select>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-1">Target Property</label>
                            <select
                                value={selectedProperty}
                                onChange={(e) => setSelectedProperty(e.target.value)}
                                disabled={!selectedObjectType}
                                className="block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md disabled:bg-gray-100 disabled:text-gray-400"
                            >
                                <option value="">Select Property...</option>
                                {selectedTypeObj?.properties?.map((p: any) => (
                                    <option key={p.id} value={p.id}>{p.id} ({p.dataType})</option>
                                ))}
                            </select>
                        </div>

                        <div>
                            <button
                                onClick={handleBind}
                                disabled={!selectedObjectType || !selectedProperty}
                                className="w-full inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                            >
                                Bind Model
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
