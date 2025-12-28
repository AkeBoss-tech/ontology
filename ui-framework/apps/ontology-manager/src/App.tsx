import React, { useState } from 'react';
import { VisualizationManagerProvider } from '@ontology/core';
import Home from './pages/Home';
import Search from './pages/Search';
import Browse from './pages/Browse';

type Page = 'home' | 'search' | 'browse';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('home');
  
  return (
    <VisualizationManagerProvider>
      <AppContent currentPage={currentPage} setCurrentPage={setCurrentPage} />
    </VisualizationManagerProvider>
  );
}

function AppContent({ currentPage, setCurrentPage }: { currentPage: Page; setCurrentPage: (page: Page) => void }) {
  return (
    <div className="min-h-screen bg-gray-50">
      <nav className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between h-16">
            <div className="flex">
              <div className="flex-shrink-0 flex items-center">
                <h1 className="text-xl font-bold">Ontology Manager</h1>
              </div>
              <div className="hidden sm:ml-6 sm:flex sm:space-x-8">
                <button
                  onClick={() => setCurrentPage('home')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'home'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Home
                </button>
                <button
                  onClick={() => setCurrentPage('search')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'search'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Search
                </button>
                <button
                  onClick={() => setCurrentPage('browse')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'browse'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Browse
                </button>
              </div>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        {currentPage === 'home' && <Home />}
        {currentPage === 'search' && <Search />}
        {currentPage === 'browse' && <Browse />}
      </main>
    </div>
  );
}

export default App;
