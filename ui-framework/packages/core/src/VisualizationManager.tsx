import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';

export interface VisualizationConfig {
  id: string;
  name: string;
  type: 'map' | 'graph' | 'chart' | 'table';
  objectType: string;
  filters?: any[];
  properties?: string[];
  settings?: Record<string, any>;
  createdAt: string;
  updatedAt: string;
}

interface VisualizationManagerContextType {
  visualizations: VisualizationConfig[];
  saveVisualization: (config: Omit<VisualizationConfig, 'id' | 'createdAt' | 'updatedAt'>) => string;
  loadVisualization: (id: string) => VisualizationConfig | undefined;
  deleteVisualization: (id: string) => void;
  updateVisualization: (id: string, updates: Partial<VisualizationConfig>) => void;
  exportVisualizations: () => string;
  importVisualizations: (json: string) => void;
}

const VisualizationManagerContext = createContext<VisualizationManagerContextType | null>(null);

const STORAGE_KEY = 'ontology_visualizations';

export function VisualizationManagerProvider({ children }: { children: ReactNode }) {
  const [visualizations, setVisualizations] = useState<VisualizationConfig[]>([]);

  // Load from localStorage on mount
  useEffect(() => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const parsed = JSON.parse(stored);
        setVisualizations(parsed);
      }
    } catch (e) {
      console.error('Failed to load visualizations from localStorage:', e);
    }
  }, []);

  // Save to localStorage whenever visualizations change
  useEffect(() => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(visualizations));
    } catch (e) {
      console.error('Failed to save visualizations to localStorage:', e);
    }
  }, [visualizations]);

  const saveVisualization = (config: Omit<VisualizationConfig, 'id' | 'createdAt' | 'updatedAt'>): string => {
    const id = `viz_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    const now = new Date().toISOString();
    const newViz: VisualizationConfig = {
      ...config,
      id,
      createdAt: now,
      updatedAt: now,
    };
    setVisualizations((prev) => [...prev, newViz]);
    return id;
  };

  const loadVisualization = (id: string): VisualizationConfig | undefined => {
    return visualizations.find((v) => v.id === id);
  };

  const deleteVisualization = (id: string) => {
    setVisualizations((prev) => prev.filter((v) => v.id !== id));
  };

  const updateVisualization = (id: string, updates: Partial<VisualizationConfig>) => {
    setVisualizations((prev) =>
      prev.map((v) =>
        v.id === id
          ? { ...v, ...updates, updatedAt: new Date().toISOString() }
          : v
      )
    );
  };

  const exportVisualizations = (): string => {
    return JSON.stringify(visualizations, null, 2);
  };

  const importVisualizations = (json: string) => {
    try {
      const parsed = JSON.parse(json) as VisualizationConfig[];
      setVisualizations((prev) => [...prev, ...parsed]);
    } catch (e) {
      throw new Error('Invalid JSON format');
    }
  };

  return (
    <VisualizationManagerContext.Provider
      value={{
        visualizations,
        saveVisualization,
        loadVisualization,
        deleteVisualization,
        updateVisualization,
        exportVisualizations,
        importVisualizations,
      }}
    >
      {children}
    </VisualizationManagerContext.Provider>
  );
}

export function useVisualizationManager() {
  const context = useContext(VisualizationManagerContext);
  if (!context) {
    throw new Error('useVisualizationManager must be used within VisualizationManagerProvider');
  }
  return context;
}





