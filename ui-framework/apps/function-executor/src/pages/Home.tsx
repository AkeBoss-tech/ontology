import React from 'react';
import { BeakerIcon, BoltIcon, CpuChipIcon } from '@heroicons/react/24/outline';

interface HomeProps {
    onNavigateToExecutor: () => void;
}

export default function Home({ onNavigateToExecutor }: HomeProps) {
    return (
        <div className="px-4 py-6 sm:px-0">
            <div className="text-center mb-12">
                <h1 className="text-4xl font-bold text-gray-900 mb-4">Function Executor</h1>
                <p className="text-xl text-gray-600 max-w-2xl mx-auto">
                    Execute ontology functions with parameters and view results in real-time
                </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-blue-100 rounded-lg mb-4">
                        <BeakerIcon className="w-6 h-6 text-blue-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Function Library</h3>
                    <p className="text-gray-600">
                        Browse all available functions defined in your ontology with their parameters and return types
                    </p>
                </div>

                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-green-100 rounded-lg mb-4">
                        <BoltIcon className="w-6 h-6 text-green-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Execute & Test</h3>
                    <p className="text-gray-600">
                        Input parameters and execute functions to see results instantly with type-safe validation
                    </p>
                </div>

                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-purple-100 rounded-lg mb-4">
                        <CpuChipIcon className="w-6 h-6 text-purple-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Cached Results</h3>
                    <p className="text-gray-600">
                        Functions marked as cacheable will return cached results for improved performance
                    </p>
                </div>
            </div>

            <div className="bg-white p-8 rounded-lg shadow-sm border mb-8">
                <h2 className="text-2xl font-bold mb-4">What are Functions?</h2>
                <div className="space-y-4 text-gray-700">
                    <p>
                        Functions in the ontology framework are reusable computational units that can:
                    </p>
                    <ul className="list-disc list-inside space-y-2 ml-4">
                        <li><strong>Aggregate data</strong> across linked objects (e.g., sum portfolio values)</li>
                        <li><strong>Traverse relationships</strong> to find connected objects</li>
                        <li><strong>Access properties</strong> from objects with type safety</li>
                        <li><strong>Return typed results</strong> that can be used in other queries</li>
                    </ul>
                    <p className="mt-4">
                        Functions are defined declaratively in your ontology YAML/JSON and executed by the backend engine.
                    </p>
                </div>
            </div>

            <div className="text-center">
                <button
                    onClick={onNavigateToExecutor}
                    className="px-6 py-3 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition-colors shadow-sm"
                >
                    Start Executing Functions
                </button>
            </div>
        </div>
    );
}
