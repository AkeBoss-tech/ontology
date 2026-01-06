import React from 'react';
import { useQuery } from '@apollo/client';
import { GET_MODEL_DETAILS } from '../graphql/model-queries';
import { ArrowLeftIcon } from '@heroicons/react/24/outline';
import ModelBinding from './ModelBinding';

interface ModelDetailProps {
    modelId: string;
    onBack: () => void;
}

export default function ModelDetail({ modelId, onBack }: ModelDetailProps) {
    const { loading, error, data } = useQuery(GET_MODEL_DETAILS, {
        variables: { id: modelId },
    });

    if (loading) return <div className="p-8 text-center text-gray-500">Loading model details...</div>;
    if (error) return <div className="p-8 text-center text-red-500">Error: {error.message}</div>;

    const model = data?.model;

    if (!model) return <div className="p-8 text-center text-gray-500">Model not found</div>;

    const renderMetric = (label: string, value?: number | null) => {
        if (value === undefined || value === null) return null;
        return (
            <div className="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6">
                <dt className="text-sm font-medium text-gray-500">{label}</dt>
                <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2 font-mono">
                    {value.toFixed(4)}
                </dd>
            </div>
        );
    };

    return (
        <div className="bg-white shadow overflow-hidden sm:rounded-lg border border-gray-200">
            <div className="px-4 py-5 sm:px-6 flex items-center border-b border-gray-200">
                <button
                    onClick={onBack}
                    className="mr-4 p-2 rounded-full hover:bg-gray-100 text-gray-500"
                >
                    <ArrowLeftIcon className="h-5 w-5" />
                </button>
                <div>
                    <h3 className="text-lg leading-6 font-medium text-gray-900">
                        {model.name} <span className="text-gray-400 font-normal">v{model.version}</span>
                    </h3>
                    <p className="mt-1 max-w-2xl text-sm text-gray-500">{model.description}</p>
                </div>
                <div className="ml-auto">
                    <span className={`px-3 py-1 inline-flex text-sm leading-5 font-semibold rounded-full bg-blue-100 text-blue-800`}>
                        {model.status}
                    </span>
                </div>
            </div>

            <div className="border-t border-gray-200">
                <dl>
                    <div className="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 border-b border-gray-100">
                        <dt className="text-sm font-medium text-gray-500">Model ID</dt>
                        <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2 font-mono text-xs">{model.id}</dd>
                    </div>
                    <div className="bg-gray-50 px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 border-b border-gray-100">
                        <dt className="text-sm font-medium text-gray-500">Type</dt>
                        <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">{model.modelType}</dd>
                    </div>
                    <div className="bg-white px-4 py-5 sm:grid sm:grid-cols-3 sm:gap-4 sm:px-6 border-b border-gray-100">
                        <dt className="text-sm font-medium text-gray-500">Platform</dt>
                        <dd className="mt-1 text-sm text-gray-900 sm:mt-0 sm:col-span-2">
                            {model.platform.platformType} ({model.platform.framework || 'Generic'})
                            {model.platform.endpointName && <div className="text-xs text-gray-500 mt-1">Endpoint: {model.platform.endpointName}</div>}
                        </dd>
                    </div>

                    {/* Metrics Section */}
                    {model.metrics && (
                        <>
                            <div className="px-4 py-3 bg-gray-100 font-medium text-sm text-gray-700 sm:px-6">Model Metrics</div>
                            {renderMetric("Accuracy", model.metrics.accuracy)}
                            {renderMetric("F1 Score", model.metrics.f1Score)}
                            {renderMetric("Precision", model.metrics.precision)}
                            {renderMetric("Recall", model.metrics.recall)}
                            {renderMetric("AUC-ROC", model.metrics.aucRoc)}
                            {renderMetric("RMSE", model.metrics.rmse)}
                            {renderMetric("MSE", model.metrics.mse)}
                            {renderMetric("MAE", model.metrics.mae)}
                            {renderMetric("R²", model.metrics.r2)}
                        </>
                    )}
                </dl>
            </div>

            {/* Binding Section */}
            <div className="p-4 bg-gray-50 border-t border-gray-200">
                <ModelBinding modelId={model.id} />
            </div>

            <div className="px-4 py-4 sm:px-6 border-t border-gray-200 bg-gray-50 flex justify-end space-x-3">
                <button className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                    Run Prediction
                </button>
                <button className="inline-flex justify-center py-2 px-4 border border-gray-300 shadow-sm text-sm font-medium rounded-md text-gray-700 bg-white hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500">
                    Compare
                </button>
            </div>
        </div>
    );
}
