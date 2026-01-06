import React, { useState } from 'react';
import { ApolloClient, InMemoryCache } from '@apollo/client';
import { OntologyProvider } from '@ontology/core';
import Home from './pages/Home';
import ObjectTypesPage from './pages/ObjectTypesPage';
import LinkTypesPage from './pages/LinkTypesPage';
import FunctionsPage from './pages/FunctionsPage';
import InterfacesPage from './pages/InterfacesPage';
import OverviewPage from './pages/OverviewPage';

// Create Apollo Client
const client = new ApolloClient({
    uri: 'http://localhost:8000/graphql',
    cache: new InMemoryCache(),
});

type Page = 'home' | 'overview' | 'object-types' | 'link-types' | 'functions' | 'interfaces';

function App() {
    const [currentPage, setCurrentPage] = useState<Page>('home');

    const navItems: { id: Page; label: string }[] = [
        { id: 'home', label: 'Home' },
        { id: 'overview', label: 'Overview' },
        { id: 'object-types', label: 'Object Types' },
        { id: 'link-types', label: 'Link Types' },
        { id: 'functions', label: 'Functions' },
        { id: 'interfaces', label: 'Interfaces' },
    ];

    return (
        <OntologyProvider client={client}>
            <div className="flex h-screen bg-light-200 text-dark-500 overflow-hidden font-sans">
                {/* Left Sidebar - Foundry Style */}
                <aside className="w-64 bg-dark-200 flex-shrink-0 flex flex-col border-r border-dark-300">
                    {/* Logo / App Name */}
                    <div className="h-14 flex items-center px-4 border-b border-dark-300">
                        <div className="w-6 h-6 bg-foundry-core rounded mr-3 flex items-center justify-center text-white font-bold text-xs">O</div>
                        <span className="text-white font-semibold tracking-wide">Ontology Manager</span>
                    </div>

                    {/* Navigation Items */}
                    <div className="flex-1 overflow-y-auto py-4 px-2 space-y-1">

                        <div className="mb-6">
                            <h3 className="px-3 text-xs font-bold text-gray-400 uppercase tracking-wider mb-2">Discover</h3>
                            <button
                                onClick={() => setCurrentPage('home')}
                                className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm transition-colors ${currentPage === 'home'
                                        ? 'bg-foundry-core text-white'
                                        : 'text-gray-300 hover:bg-dark-100 hover:text-white'
                                    }`}
                            >
                                <span className="mr-3">🏠</span> Home
                            </button>
                            <button
                                onClick={() => setCurrentPage('overview')}
                                className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm transition-colors ${currentPage === 'overview'
                                        ? 'bg-foundry-core text-white'
                                        : 'text-gray-300 hover:bg-dark-100 hover:text-white'
                                    }`}
                            >
                                <span className="mr-3">📊</span> Dashboard
                            </button>
                        </div>

                        <div>
                            <h3 className="px-3 text-xs font-bold text-gray-400 uppercase tracking-wider mb-2">Resources</h3>
                            <button
                                onClick={() => setCurrentPage('object-types')}
                                className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm transition-colors ${currentPage === 'object-types'
                                        ? 'bg-dark-300 text-foundry-blue border-l-2 border-foundry-blue'
                                        : 'text-gray-300 hover:bg-dark-100 hover:text-white border-l-2 border-transparent'
                                    }`}
                            >
                                <span className="mr-3 opacity-70">📦</span> Object Types
                            </button>
                            <button
                                onClick={() => setCurrentPage('link-types')}
                                className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm transition-colors ${currentPage === 'link-types'
                                        ? 'bg-dark-300 text-foundry-blue border-l-2 border-foundry-blue'
                                        : 'text-gray-300 hover:bg-dark-100 hover:text-white border-l-2 border-transparent'
                                    }`}
                            >
                                <span className="mr-3 opacity-70">🔗</span> Link Types
                            </button>
                            <button
                                onClick={() => setCurrentPage('functions')}
                                className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm transition-colors ${currentPage === 'functions'
                                        ? 'bg-dark-300 text-foundry-blue border-l-2 border-foundry-blue'
                                        : 'text-gray-300 hover:bg-dark-100 hover:text-white border-l-2 border-transparent'
                                    }`}
                            >
                                <span className="mr-3 opacity-70">⚡</span> Functions
                            </button>
                            <button
                                onClick={() => setCurrentPage('interfaces')}
                                className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm transition-colors ${currentPage === 'interfaces'
                                        ? 'bg-dark-300 text-foundry-blue border-l-2 border-foundry-blue'
                                        : 'text-gray-300 hover:bg-dark-100 hover:text-white border-l-2 border-transparent'
                                    }`}
                            >
                                <span className="mr-3 opacity-70">📋</span> Interfaces
                            </button>
                        </div>
                    </div>

                    {/* Bottom Profile Section */}
                    <div className="p-4 border-t border-dark-300">
                        <div className="flex items-center">
                            <div className="w-8 h-8 rounded-full bg-gray-500 flex items-center justify-center text-xs text-white font-bold">
                                AD
                            </div>
                            <div className="ml-3">
                                <p className="text-sm font-medium text-white">Admin User</p>
                                <p className="text-xs text-gray-400">Workspace Owner</p>
                            </div>
                        </div>
                    </div>
                </aside>

                {/* Main Content Area */}
                <main className="flex-1 overflow-auto bg-light-200">
                    {/* Top Header Bar */}
                    <header className="h-14 bg-white border-b border-light-300 flex items-center justify-between px-6 sticky top-0 z-10">
                        <div className="flex items-center text-sm breadcrumbs text-dark-300">
                            <span className="text-gray-400">Ontology</span>
                            <span className="mx-2 text-gray-400">/</span>
                            <span className="font-semibold text-dark-400 capitalize">
                                {navItems.find(n => n.id === currentPage)?.label || 'Page'}
                            </span>
                        </div>
                        <div className="flex items-center space-x-4">
                            <div className="relative">
                                <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400">🔍</span>
                                <input
                                    type="text"
                                    placeholder="Search..."
                                    className="pl-9 pr-4 py-1.5 text-sm border border-light-300 rounded-sm bg-light-200 focus:outline-none focus:border-foundry-core focus:ring-1 focus:ring-foundry-core w-64 transition-all"
                                />
                            </div>
                            <button className="text-sm font-medium text-foundry-core hover:text-foundry-hover">
                                + New
                            </button>
                        </div>
                    </header>

                    <div className="p-6 max-w-7xl mx-auto">
                        {currentPage === 'home' && <Home onNavigate={setCurrentPage} />}
                        {currentPage === 'overview' && <OverviewPage />}
                        {currentPage === 'object-types' && <ObjectTypesPage />}
                        {currentPage === 'link-types' && <LinkTypesPage />}
                        {currentPage === 'functions' && <FunctionsPage />}
                        {currentPage === 'interfaces' && <InterfacesPage />}
                    </div>
                </main>
            </div>
        </OntologyProvider>
    );
}

export default App;
