import { gql } from '@apollo/client';

export const GET_ONTOLOGY_STRUCTURE = gql`
  query GetOntologyStructure {
    ontology {
      objectTypes {
        id
        properties {
          id
          dataType
        }
      }
    }
  }
`;
