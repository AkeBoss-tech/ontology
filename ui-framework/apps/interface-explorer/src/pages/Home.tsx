import React from 'react';
import { Square3Stack3DIcon, MagnifyingGlassIcon, LinkIcon } from '@heroicons/react/24/outline';

interface HomeProps {
    onNavigateToExplorer: () => void;
}

export default function Home({ onNavigateToExplorer }: HomeProps) {
    return (
        <div className="px-4 py-6 sm:px-0">
            <div className="text-center mb-12">
                <h1 className="text-4xl font-bold text-gray-900 mb-4">Interface Explorer</h1>
                <p className="text-xl text-gray-600 max-w-2xl mx-auto">
                    Query objects polymorphically by interface to find all implementations across your ontology
                </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-blue-100 rounded-lg mb-4">
                        <Square3Stack3DIcon className="w-6 h-6 text-blue-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Interface Definitions</h3>
                    <p className="text-gray-600">
                        Browse all interfaces defined in your ontology with their required properties and link types
                    </p>
                </div>

                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-green-100 rounded-lg mb-4">
                        <MagnifyingGlassIcon className="w-6 h-6 text-green-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Polymorphic Queries</h3>
                    <p className="text-gray-600">
                        Query all objects that implement an interface, regardless of their specific object type
                    </p>
                </div>

                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-purple-100 rounded-lg mb-4">
                        <LinkIcon className="w-6 h-6 text-purple-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Implementers</h3>
                    <p className="text-gray-600">
                        See which object types implement each interface and how many instances exist
                    </p>
                </div>
            </div>

            <div className="bg-white p-8 rounded-lg shadow-sm border mb-8">
                <h2 className="text-2xl font-bold mb-4">What are Interfaces?</h2>
                <div className="space-y-4 text-gray-700">
                    <p>
                        Interfaces in the ontology framework enable polymorphic data modeling by defining contracts that multiple object types can implement:
                    </p>
                    <ul className="list-disc list-inside space-y-2 ml-4">
                        <li><strong>Define common properties</strong> that all implementers must have</li>
                        <li><strong>Enable polymorphic queries</strong> across different object types</li>
                        <li><strong>Promote code reuse</strong> by abstracting common behavior</li>
                        <li><strong>Support multiple inheritance</strong> - objects can implement multiple interfaces</li>
                    </ul>

                    <div className="mt-6 p-4 bg-gray-50 rounded border">
                        <h4 className="font-semibold mb-2">Example: Location Interface</h4>
                        <p className="text-sm mb-2">
                            An interface called "Location" might require properties like <code className="bg-gray-200 px-1 rounded">latitude</code> and <code className="bg-gray-200 px-1 rounded">longitude</code>.
                        </p>
                        <p className="text-sm">
                            Object types like "Office", "Warehouse", and "Store" could all implement the Location interface, allowing you to query all locations regardless of their specific type.
                        </p>
                    </div>
                </div>
            </div>

            <div className="text-center">
                <button
                    onClick={onNavigateToExplorer}
                    className="px-6 py-3 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition-colors shadow-sm"
                >
                    Start Exploring Interfaces
                </button>
            </div>
        </div>
    );
}
