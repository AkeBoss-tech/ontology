import React from 'react';

// Note: Link types might not be exposed via GraphQL yet
// This page serves as a placeholder and documentation

export default function LinkTypesPage() {
    // Placeholder data - would come from GraphQL query
    const linkTypesInfo = [
        {
            title: 'What are Link Types?',
            content: 'Link types define relationships between object types, such as "Customer places Order" or "Product belongs to Category".'
        },
        {
            title: 'Relationship Cardinality',
            content: 'Links can be one-to-one, one-to-many, or many-to-many. The ontology enforces these relationships.'
        },
        {
            title: 'Traversal',
            content: 'Links enable graph traversal queries, allowing you to navigate from one object to related objects.'
        }
    ];

    return (
        <div className="px-4 py-6 sm:px-0">
            <h1 className="text-2xl font-bold mb-6">Link Types</h1>
            <p className="text-gray-600 mb-6">
                Link types define how objects are connected to each other, enabling relationship-based queries and graph traversal.
            </p>

            <div className="bg-yellow-50 border border-yellow-200 rounded-lg p-4 mb-6">
                <div className="flex items-start gap-2">
                    <span className="text-yellow-600 text-xl">ℹ️</span>
                    <div>
                        <h3 className="font-semibold text-yellow-800">Link Types Overview</h3>
                        <p className="text-sm text-yellow-700 mt-1">
                            Link types are defined in your ontology configuration and establish relationships between object types.
                            To view specific links, use the Object Explorer to navigate between related objects.
                        </p>
                    </div>
                </div>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                {linkTypesInfo.map((info, idx) => (
                    <div key={idx} className="bg-white rounded-lg shadow-sm border p-6">
                        <h3 className="font-semibold text-lg mb-2 text-green-700">{info.title}</h3>
                        <p className="text-gray-600 text-sm">{info.content}</p>
                    </div>
                ))}
            </div>

            <div className="bg-white rounded-lg shadow-sm border p-6">
                <h2 className="font-semibold text-lg mb-4">Link Type Concepts</h2>
                <div className="space-y-4">
                    <div className="flex items-start gap-4">
                        <div className="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center flex-shrink-0">
                            <span className="text-green-600 font-bold">→</span>
                        </div>
                        <div>
                            <h4 className="font-medium">Source → Target</h4>
                            <p className="text-sm text-gray-600">
                                Each link type has a source object type and a target object type.
                                For example: <code className="bg-gray-100 px-1 rounded">Customer</code> →
                                <code className="bg-gray-100 px-1 rounded">Order</code>
                            </p>
                        </div>
                    </div>
                    <div className="flex items-start gap-4">
                        <div className="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center flex-shrink-0">
                            <span className="text-green-600 font-bold">↔</span>
                        </div>
                        <div>
                            <h4 className="font-medium">Bidirectional Navigation</h4>
                            <p className="text-sm text-gray-600">
                                Links can often be traversed in both directions. If a customer "places" orders,
                                you can also find which customer an order "belongs to".
                            </p>
                        </div>
                    </div>
                    <div className="flex items-start gap-4">
                        <div className="w-8 h-8 bg-green-100 rounded-full flex items-center justify-center flex-shrink-0">
                            <span className="text-green-600 font-bold">#</span>
                        </div>
                        <div>
                            <h4 className="font-medium">Link Properties</h4>
                            <p className="text-sm text-gray-600">
                                Links can have their own properties, like a "created_at" timestamp
                                on a relationship or a "role" on a team membership.
                            </p>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    );
}
