import React from 'react';
import ReactDOM from 'react-dom/client';
import { ApolloProvider } from '@apollo/client';
import { OntologyProvider, createOntologyClient } from '@ontology/core';
import App from './App';
import './index.css';

const graphqlUrl = import.meta.env.VITE_GRAPHQL_URL || '/graphql';
const client = createOntologyClient(graphqlUrl);

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ApolloProvider client={client}>
      <OntologyProvider client={client}>
        <App />
      </OntologyProvider>
    </ApolloProvider>
  </React.StrictMode>
);


