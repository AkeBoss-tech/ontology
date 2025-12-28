import React, { useState } from 'react';
import TractMap from './pages/TractMap';
import PUMSAnalysis from './pages/PUMSAnalysis';
import CrosswalkView from './pages/CrosswalkView';
import CohortBuilder from './pages/CohortBuilder';
import EnhancedMap from './pages/EnhancedMap';
import EnhancedPersonSearch from './pages/EnhancedPersonSearch';
import DataSourceManager from './components/DataSourceManager';
import VisualizationManager from './components/VisualizationManager';
import { VisualizationManagerProvider } from '@ontology/core';

type Page = 'tract-map' | 'pums-analysis' | 'crosswalk' | 'cohort' | 'data-sources' | 'visualizations' | 'enhanced-map' | 'person-search';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('tract-map');
  
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
                <h1 className="text-xl font-bold">Census Data Explorer</h1>
              </div>
              <div className="hidden sm:ml-6 sm:flex sm:space-x-8">
                <button
                  onClick={() => setCurrentPage('tract-map')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'tract-map'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Tract Map
                </button>
                <button
                  onClick={() => setCurrentPage('pums-analysis')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'pums-analysis'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  PUMS Analysis
                </button>
                <button
                  onClick={() => setCurrentPage('crosswalk')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'crosswalk'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Crosswalk
                </button>
                <button
                  onClick={() => setCurrentPage('cohort')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'cohort'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Cohort Builder
                </button>
                <button
                  onClick={() => setCurrentPage('data-sources')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'data-sources'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Data Sources
                </button>
                <button
                  onClick={() => setCurrentPage('visualizations')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'visualizations'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Visualizations
                </button>
                <button
                  onClick={() => setCurrentPage('enhanced-map')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'enhanced-map'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Interactive Map
                </button>
                <button
                  onClick={() => setCurrentPage('person-search')}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'person-search'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Person Search
                </button>
              </div>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
        {currentPage === 'tract-map' && <TractMap />}
        {currentPage === 'pums-analysis' && <PUMSAnalysis />}
        {currentPage === 'crosswalk' && <CrosswalkView />}
        {currentPage === 'cohort' && <CohortBuilder />}
        {currentPage === 'data-sources' && <DataSourceManager />}
        {currentPage === 'visualizations' && <VisualizationManager />}
        {currentPage === 'enhanced-map' && <EnhancedMap />}
        {currentPage === 'person-search' && <EnhancedPersonSearch />}
      </main>
    </div>
  );
}

export default App;

