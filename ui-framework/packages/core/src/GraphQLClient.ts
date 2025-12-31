import { ApolloClient, InMemoryCache, createHttpLink } from '@apollo/client';

export function createOntologyClient(uri: string = '/graphql') {
  return new ApolloClient({
    link: createHttpLink({ uri }),
    cache: new InMemoryCache(),
  });
}

export const gql = (query: TemplateStringsArray) => query.join('');





