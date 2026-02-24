import React, { createContext, useContext, ReactNode, useState, useEffect } from 'react';
import { ApolloClient } from '@apollo/client';

export const AVAILABLE_ONTOLOGIES = [
  { id: 'default', name: 'Default Ontology' },
  { id: 'manufacturing', name: 'Manufacturing' },
  { id: 'logistics', name: 'Logistics' },
];

interface OntologyContextType {
  client: ApolloClient<any>;
  ontologyUrl?: string;
  currentOntologyId: string;
  setOntologyId: (id: string) => void;
  availableOntologies: typeof AVAILABLE_ONTOLOGIES;
}

const OntologyContext = createContext<OntologyContextType | null>(null);

export interface OntologyProviderProps {
  client: ApolloClient<any>;
  ontologyUrl?: string;
  children: ReactNode;
}

export function OntologyProvider({ client, ontologyUrl, children }: OntologyProviderProps) {
  const [currentOntologyId, setCurrentOntologyId] = useState<string>(() => {
    return localStorage.getItem('ontology_platform_current_ontology') || 'default';
  });

  useEffect(() => {
    localStorage.setItem('ontology_platform_current_ontology', currentOntologyId);
  }, [currentOntologyId]);

  return (
    <OntologyContext.Provider value={{
      client,
      ontologyUrl,
      currentOntologyId,
      setOntologyId: setCurrentOntologyId,
      availableOntologies: AVAILABLE_ONTOLOGIES
    }}>
      {children}
    </OntologyContext.Provider>
  );
}

export function useOntology() {
  const context = useContext(OntologyContext);
  if (!context) {
    throw new Error('useOntology must be used within OntologyProvider');
  }
  return context;
}







