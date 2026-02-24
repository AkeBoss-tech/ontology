import React, { useState } from 'react';
import { ApolloClient, InMemoryCache } from '@apollo/client';
import { OntologyProvider } from '@ontology/core';
import Home from './pages/Home';
import Explorer from './pages/Explorer';
import ObjectDetail from './pages/ObjectDetail';

// Create Apollo Client (Shared with other apps)
const client = new ApolloClient({
  uri: 'http://localhost:8000/graphql',
  cache: new InMemoryCache(),
});

type Page = 'home' | 'explorer' | 'detail';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('home');
  const [selectedObject, setSelectedObject] = useState<{ type: string; id: string } | null>(null);

  // Auto-navigate to detail if object selected
  React.useEffect(() => {
    if (selectedObject) {
      setCurrentPage('detail');
    }
  }, [selectedObject]);

  return (
    <OntologyProvider client={client}>
      <div className="flex h-screen bg-light-200 text-dark-500 overflow-hidden font-sans">
        {/* Left Sidebar - Foundry Style */}
        <aside className="w-64 bg-dark-200 flex-shrink-0 flex flex-col border-r border-dark-300">
          {/* Logo / App Name */}
          <div className="h-14 flex items-center px-4 border-b border-dark-300">
            <div className="w-6 h-6 bg-foundry-core rounded mr-3 flex items-center justify-center text-white font-bold text-xs">O</div>
            <span className="text-white font-semibold tracking-wide">Object Explorer</span>
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
                onClick={() => {
                  setCurrentPage('explorer');
                  setSelectedObject(null);
                }}
                className={`w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm transition-colors ${currentPage === 'explorer' || currentPage === 'detail'
                    ? 'bg-foundry-core text-white'
                    : 'text-gray-300 hover:bg-dark-100 hover:text-white'
                  }`}
              >
                <span className="mr-3">🔍</span> Explorer
              </button>
            </div>

            <div>
              <h3 className="px-3 text-xs font-bold text-gray-400 uppercase tracking-wider mb-2">Saved Queries</h3>
              <button className="w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm text-gray-300 hover:bg-dark-100 hover:text-white transition-colors">
                <span className="mr-3 opacity-70">📂</span> Recent Searches
              </button>
              <button className="w-full flex items-center px-3 py-2 text-sm font-medium rounded-sm text-gray-300 hover:bg-dark-100 hover:text-white transition-colors">
                <span className="mr-3 opacity-70">⭐</span> Favorites
              </button>
            </div>

            <div className="mt-6">
              <h3 className="px-3 text-xs font-bold text-gray-400 uppercase tracking-wider mb-2">Workspaces</h3>
              <div className="px-3 py-2 text-sm text-gray-400 italic">No active workspaces</div>
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
        <main className="flex-1 overflow-auto bg-light-200 flex flex-col">
          {/* Top Header Bar */}
          <header className="h-14 bg-white border-b border-light-300 flex items-center justify-between px-6 sticky top-0 z-10 flex-shrink-0">
            <div className="flex items-center text-sm breadcrumbs text-dark-300">
              <span className="text-gray-400">Object Explorer</span>
              <span className="mx-2 text-gray-400">/</span>
              <span className="font-semibold text-dark-400 capitalize">
                {currentPage === 'detail' ? `Object: ${selectedObject?.id}` : currentPage}
              </span>
            </div>
            <div className="flex items-center space-x-4">
              <div className="relative">
                <span className="absolute inset-y-0 left-0 flex items-center pl-3 text-gray-400">🔍</span>
                <input
                  type="text"
                  placeholder="Search objects..."
                  className="pl-9 pr-4 py-1.5 text-sm border border-light-300 rounded-sm bg-light-200 focus:outline-none focus:border-foundry-core focus:ring-1 focus:ring-foundry-core w-64 transition-all"
                />
              </div>
              <button className="text-sm font-medium text-foundry-core hover:text-foundry-hover">
                + Actions
              </button>
            </div>
          </header>

          <div className="flex-1 p-6 relative">
            {currentPage === 'home' && <Home onNavigateToExplorer={() => setCurrentPage('explorer')} />}
            {currentPage === 'explorer' && <Explorer onSelectObject={setSelectedObject} />}
            {currentPage === 'detail' && selectedObject && (
              <ObjectDetail
                objectType={selectedObject.type}
                objectId={selectedObject.id}
                onBack={() => {
                  setCurrentPage('explorer');
                  setSelectedObject(null);
                }}
              />
            )}
          </div>
        </main>
      </div>
    </OntologyProvider>
  );
}

export default App;