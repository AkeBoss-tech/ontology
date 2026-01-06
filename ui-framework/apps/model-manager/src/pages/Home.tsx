import React from 'react';
import { QueueListIcon, ChartBarIcon, ArrowPathIcon } from '@heroicons/react/24/outline';

interface HomeProps {
    onNavigateToRegistry: () => void;
}

export default function Home({ onNavigateToRegistry }: HomeProps) {
    return (
        <div className="px-4 py-6 sm:px-0">
            <div className="text-center mb-12">
                <h1 className="text-4xl font-bold text-gray-900 mb-4">Model Manager</h1>
                <p className="text-xl text-gray-600 max-w-2xl mx-auto">
                    Centralized registry for managing, comparing, and deploying machine learning models within your ontology
                </p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-12">
                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-blue-100 rounded-lg mb-4">
                        <QueueListIcon className="w-6 h-6 text-blue-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Model Registry</h3>
                    <p className="text-gray-600">
                        View and manage all registered models, track versions, and monitor lifecycle status
                    </p>
                </div>

                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-green-100 rounded-lg mb-4">
                        <ChartBarIcon className="w-6 h-6 text-green-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Metrics & Comparison</h3>
                    <p className="text-gray-600">
                        Compare model performance side-by-side using key metrics like Accuracy, F1 Score, and RMSE
                    </p>
                </div>

                <div className="bg-white p-6 rounded-lg shadow-sm border">
                    <div className="flex items-center justify-center w-12 h-12 bg-purple-100 rounded-lg mb-4">
                        <ArrowPathIcon className="w-6 h-6 text-purple-600" />
                    </div>
                    <h3 className="text-lg font-semibold mb-2">Property Binding</h3>
                    <p className="text-gray-600">
                        Directly bind ML models to ontology properties to power predictive features in your applications
                    </p>
                </div>
            </div>

            <div className="bg-white p-8 rounded-lg shadow-sm border mb-8">
                <h2 className="text-2xl font-bold mb-4">Why use Model Manager?</h2>
                <div className="space-y-4 text-gray-700">
                    <p>
                        The Model Manager bridges the gap between data science and operational applications:
                    </p>
                    <ul className="list-disc list-inside space-y-2 ml-4">
                        <li><strong>Standardized Registry:</strong> Keep track of all models across different frameworks (sklearn, PyTorch, TensorFlow)</li>
                        <li><strong>Performance Tracking:</strong> Ensure only the best performing models are promoted to production</li>
                        <li><strong>Easy Integration:</strong> Bind models to object properties with a few clicks, no application code changes required</li>
                        <li><strong>Lifecycle Management:</strong> strict versioning and status control (Training → Registered → Bound → Deprecated)</li>
                    </ul>
                </div>
            </div>

            <div className="text-center">
                <button
                    onClick={onNavigateToRegistry}
                    className="px-6 py-3 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 transition-colors shadow-sm"
                >
                    Open Registry
                </button>
            </div>
        </div>
    );
}
