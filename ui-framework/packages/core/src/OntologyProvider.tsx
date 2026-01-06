import React, { createContext, useContext, ReactNode } from 'react';
import { ApolloClient } from '@apollo/client';

interface OntologyContextType {
  client: ApolloClient<any>;
  ontologyUrl?: string;
}

const OntologyContext = createContext<OntologyContextType | null>(null);

export interface OntologyProviderProps {
  client: ApolloClient<any>;
  ontologyUrl?: string;
  children: ReactNode;
}

export function OntologyProvider({ client, ontologyUrl, children }: OntologyProviderProps) {
  return (
    <OntologyContext.Provider value={{ client, ontologyUrl }}>
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







