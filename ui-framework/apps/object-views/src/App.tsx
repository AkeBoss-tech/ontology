import React, { useState } from 'react';
import { VisualizationManagerProvider } from '@ontology/core';
import Home from './pages/Home';
import ViewBuilder from './pages/ViewBuilder';
import SavedViews from './pages/SavedViews';

type Page = 'home' | 'builder' | 'views';

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
                <h1 className="text-xl font-bold">Object Views</h1>
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
                  onClick={() => setCurrentPage('builder')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'builder'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  View Builder
                </button>
                <button
                  onClick={() => setCurrentPage('views')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'views'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Saved Views
                </button>
              </div>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        {currentPage === 'home' && <Home />}
        {currentPage === 'builder' && <ViewBuilder />}
        {currentPage === 'views' && <SavedViews />}
      </main>
    </div>
  );
}

export default App;