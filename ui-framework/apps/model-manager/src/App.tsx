import React, { useState } from 'react';
import { ApolloClient, InMemoryCache } from '@apollo/client';
import { OntologyProvider } from '@ontology/core';
import Home from './pages/Home';
import Registry from './pages/Registry';

// Create Apollo Client
const client = new ApolloClient({
    uri: 'http://localhost:8000/graphql',
    cache: new InMemoryCache(),
});

type Page = 'home' | 'registry';

function App() {
    const [currentPage, setCurrentPage] = useState<Page>('home');

    return (
        <OntologyProvider client={client}>
            <div className="min-h-screen bg-gray-50">
                {/* Navigation */}
                <nav className="bg-white shadow-sm border-b">
                    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        <div className="flex justify-between h-16">
                            <div className="flex">
                                <div className="flex-shrink-0 flex items-center">
                                    <h1 className="text-xl font-bold text-gray-900">Model Manager</h1>
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
                                        onClick={() => setCurrentPage('registry')}
                                        className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${currentPage === 'registry'
                                            ? 'border-blue-500 text-gray-900'
                                            : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                                            }`}
                                    >
                                        Registry
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </nav>

                {/* Main Content */}
                <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                    {currentPage === 'home' && <Home onNavigateToRegistry={() => setCurrentPage('registry')} />}
                    {currentPage === 'registry' && <Registry />}
                </main>
            </div>
        </OntologyProvider>
    );
}

export default App;
