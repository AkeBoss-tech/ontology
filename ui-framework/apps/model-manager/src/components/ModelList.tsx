import React from 'react';
import { useQuery } from '@apollo/client';
import { GET_MODELS } from '../graphql/model-queries';
import { ChevronRightIcon } from '@heroicons/react/24/outline';

interface ModelListProps {
    onSelectModel: (id: string) => void;
}

export default function ModelList({ onSelectModel }: ModelListProps) {
    const { loading, error, data } = useQuery(GET_MODELS);

    if (loading) return <div className="p-4 text-center text-gray-500">Loading models...</div>;
    if (error) return <div className="p-4 text-center text-red-500">Error loading models: {error.message}</div>;

    const models = data?.models || [];

    const getStatusColor = (status: string) => {
        switch (status) {
            case 'REGISTERED': return 'bg-blue-100 text-blue-800';
            case 'TRAINING': return 'bg-yellow-100 text-yellow-800';
            case 'BOUND': return 'bg-green-100 text-green-800';
            case 'DEPRECATED': return 'bg-orange-100 text-orange-800';
            case 'ARCHIVED': return 'bg-gray-100 text-gray-800';
            default: return 'bg-gray-100 text-gray-800';
        }
    };

    return (
        <div className="bg-white shadow overflow-hidden sm:rounded-md border border-gray-200">
            <div className="px-4 py-5 sm:px-6 border-b border-gray-200 flex justify-between items-center">
                <h3 className="text-lg leading-6 font-medium text-gray-900">Registered Models</h3>
                <span className="inline-flex items-center px-3 py-0.5 rounded-full text-sm font-medium bg-gray-100 text-gray-800">
                    {models.length} Models
                </span>
            </div>

            {models.length === 0 ? (
                <div className="p-8 text-center text-gray-500">
                    No models registered correctly. Use the "Register Model" button to add one.
                </div>
            ) : (
                <ul role="list" className="divide-y divide-gray-200">
                    {models.map((model: any) => (
                        <li key={model.id}>
                            <a
                                href="#"
                                onClick={(e) => { e.preventDefault(); onSelectModel(model.id); }}
                                className="block hover:bg-gray-50 transition duration-150 ease-in-out"
                            >
                                <div className="px-4 py-4 sm:px-6">
                                    <div className="flex items-center justify-between">
                                        <div className="flex items-center truncate">
                                            <p className="text-sm font-medium text-blue-600 truncate">{model.name}</p>
                                            <span className="ml-2 px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-gray-100 text-gray-800">
                                                v{model.version}
                                            </span>
                                        </div>
                                        <div className="ml-2 flex-shrink-0 flex">
                                            <span className={`px-2 inline-flex text-xs leading-5 font-semibold rounded-full ${getStatusColor(model.status)}`}>
                                                {model.status}
                                            </span>
                                        </div>
                                    </div>
                                    <div className="mt-2 sm:flex sm:justify-between">
                                        <div className="sm:flex">
                                            <p className="flex items-center text-sm text-gray-500 mr-6">
                                                <span className="font-semibold mr-1">Type:</span> {model.modelType}
                                            </p>
                                            <p className="flex items-center text-sm text-gray-500">
                                                <span className="font-semibold mr-1">Framework:</span> {model.platform.framework || 'N/A'}
                                            </p>
                                        </div>
                                        <div className="mt-2 flex items-center text-sm text-gray-500 sm:mt-0">
                                            <p>
                                                Created on <time dateTime={model.createdAt}>{new Date(model.createdAt).toLocaleDateString()}</time>
                                            </p>
                                            <ChevronRightIcon className="flex-shrink-0 ml-2 h-5 w-5 text-gray-400" aria-hidden="true" />
                                        </div>
                                    </div>
                                </div>
                            </a>
                        </li>
                    ))}
                </ul>
            )}
        </div>
    );
}
