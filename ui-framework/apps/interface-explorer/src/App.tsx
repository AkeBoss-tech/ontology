import React, { useState } from 'react';
import { ApolloClient, InMemoryCache } from '@apollo/client';
import { InterfaceQuery, OntologyProvider } from '@ontology/core';
import Home from './pages/Home';

// Create Apollo Client
const client = new ApolloClient({
    uri: 'http://localhost:8000/graphql',
    cache: new InMemoryCache(),
});

type Page = 'home' | 'explorer';

function App() {
    const [currentPage, setCurrentPage] = useState<Page>('home');
    const [selectedObject, setSelectedObject] = useState<{ id: string; type: string } | null>(null);

    const handleSelectObject = (objectId: string, objectType: string) => {
        setSelectedObject({ id: objectId, type: objectType });
        // In a real app, you might navigate to an object detail page here
        console.log('Selected object:', { objectId, objectType });
    };

    return (
        <OntologyProvider client={client}>
            <div className="min-h-screen bg-gray-50">
                {/* Navigation */}
                <nav className="bg-white shadow-sm border-b">
                    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        <div className="flex justify-between h-16">
                            <div className="flex">
                                <div className="flex-shrink-0 flex items-center">
                                    <h1 className="text-xl font-bold text-gray-900">Interface Explorer</h1>
                                </div>
                                <div className="hidden sm:ml-6 sm:flex sm:space-x-8">
                                    <button
                                        onClick={() => setCurrentPage('home')}
                                        className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${currentPage === 'home'
                                            ? 'border-blue-500 text-gray-900'
                                            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                                            }`}
                                    >
                                        Home
                                    </button>
                                    <button
                                        onClick={() => {
                                            setCurrentPage('explorer');
                                            setSelectedObject(null);
                                        }}
                                        className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${currentPage === 'explorer'
                                            ? 'border-blue-500 text-gray-900'
                                            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                                            }`}
                                    >
                                        Query by Interface
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </nav>

                {/* Main Content */}
                <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                    {currentPage === 'home' && <Home onNavigateToExplorer={() => setCurrentPage('explorer')} />}
                    {currentPage === 'explorer' && (
                        <div className="px-4 py-6 sm:px-0">
                            <InterfaceQuery onSelectObject={handleSelectObject} />

                            {selectedObject && (
                                <div className="mt-6 p-4 bg-blue-50 border border-blue-200 rounded-lg">
                                    <h3 className="font-semibold text-blue-900 mb-2">Selected Object</h3>
                                    <p className="text-sm text-blue-800">
                                        <span className="font-medium">Type:</span> {selectedObject.type}
                                    </p>
                                    <p className="text-sm text-blue-800">
                                        <span className="font-medium">ID:</span> {selectedObject.id}
                                    </p>
                                    <p className="text-xs text-blue-600 mt-2">
                                        (In a full implementation, this would navigate to the object detail page)
                                    </p>
                                </div>
                            )}
                        </div>
                    )}
                </main>
            </div>
        </OntologyProvider>
    );
}

export default App;
