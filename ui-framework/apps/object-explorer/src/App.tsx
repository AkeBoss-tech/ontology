import React, { useState } from 'react';
import { VisualizationManagerProvider } from '@ontology/core';
import Home from './pages/Home';
import Explorer from './pages/Explorer';
import ObjectDetail from './pages/ObjectDetail';

type Page = 'home' | 'explorer' | 'detail';

function App() {
  const [currentPage, setCurrentPage] = useState<Page>('home');
  const [selectedObject, setSelectedObject] = useState<{ type: string; id: string } | null>(null);
  
  return (
    <VisualizationManagerProvider>
      <AppContent 
        currentPage={currentPage} 
        setCurrentPage={setCurrentPage}
        selectedObject={selectedObject}
        setSelectedObject={setSelectedObject}
      />
    </VisualizationManagerProvider>
  );
}

function AppContent({ 
  currentPage, 
  setCurrentPage,
  selectedObject,
  setSelectedObject 
}: { 
  currentPage: Page; 
  setCurrentPage: (page: Page) => void;
  selectedObject: { type: string; id: string } | null;
  setSelectedObject: (obj: { type: string; id: string } | null) => void;
}) {
  // Navigate to detail page when object is selected
  React.useEffect(() => {
    if (selectedObject) {
      setCurrentPage('detail');
    }
  }, [selectedObject, setCurrentPage]);

  return (
    <div className="min-h-screen bg-gray-50">
      <nav className="bg-white shadow-sm border-b">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex justify-between h-16">
            <div className="flex">
              <div className="flex-shrink-0 flex items-center">
                <h1 className="text-xl font-bold">Object Explorer</h1>
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
                  onClick={() => {
                    setCurrentPage('explorer');
                    setSelectedObject(null);
                  }}
                  className={`inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium ${
                    currentPage === 'explorer'
                      ? 'border-blue-500 text-gray-900'
                      : 'border-transparent text-gray-500 hover:text-gray-700 hover:border-gray-300'
                  }`}
                >
                  Explorer
                </button>
              </div>
            </div>
          </div>
        </div>
      </nav>

      <main className="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
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
      </main>
    </div>
  );
}

export default App;