import React, { useState } from 'react';
import ModelList from '../components/ModelList';
import ModelDetail from '../components/ModelDetail';

export default function Registry() {
    const [selectedModelId, setSelectedModelId] = useState<string | null>(null);

    return (
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
            <div className="mb-8">
                <h1 className="text-3xl font-bold text-gray-900">Model Registry</h1>
                <p className="mt-2 text-sm text-gray-700">
                    Manage and monitor machine learning models registered in the ontology.
                </p>
            </div>

            {selectedModelId ? (
                <ModelDetail
                    modelId={selectedModelId}
                    onBack={() => setSelectedModelId(null)}
                />
            ) : (
                <ModelList onSelectModel={setSelectedModelId} />
            )}
        </div>
    );
}
