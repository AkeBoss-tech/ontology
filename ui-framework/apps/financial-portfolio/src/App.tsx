import React, { useState } from 'react';
import { VisualizationManagerProvider } from '@ontology/core';
import Home from './pages/Home';
import PortfolioBrowser from './pages/PortfolioBrowser';
import AssetSearch from './pages/AssetSearch';
import Transactions from './pages/Transactions';

type Page = 'home' | 'portfolios' | 'assets' | 'transactions';

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
                <h1 className="text-xl font-bold">Financial Portfolio Manager</h1>
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
                  onClick={() => setCurrentPage('portfolios')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'portfolios'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Portfolios
                </button>
                <button
                  onClick={() => setCurrentPage('assets')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'assets'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Assets
                </button>
                <button
                  onClick={() => setCurrentPage('transactions')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'transactions'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Transactions
                </button>
              </div>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        {currentPage === 'home' && <Home />}
        {currentPage === 'portfolios' && <PortfolioBrowser />}
        {currentPage === 'assets' && <AssetSearch />}
        {currentPage === 'transactions' && <Transactions />}
      </main>
    </div>
  );
}

export default App;
