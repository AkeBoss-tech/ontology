import { gql } from '@apollo/client';

export const GET_MODELS = gql`
  query GetModels($modelType: String, $status: String, $limit: Int, $offset: Int) {
    models(modelType: $modelType, status: $status, limit: $limit, offset: $offset) {
      id
      name
      version
      description
      modelType
      status
      platform {
        platformType
        endpointName
        framework
      }
      metrics {
        accuracy
        f1Score
        rmse
        mse
        mae
        r2
      }
      createdAt
      updatedAt
    }
  }
`;

export const GET_MODEL_DETAILS = gql`
  query GetModelDetails($id: String!) {
    model(id: $id) {
      id
      name
      version
      description
      modelType
      status
      platform {
        platformType
        endpointName
        framework
      }
      metrics {
        accuracy
        precision
        recall
        f1Score
        aucRoc
        rmse
        mse
        mae
        r2
        mape
        silhouetteScore
        daviesBouldinIndex
        custom
      }
      createdAt
      updatedAt
    }
  }
`;

export const REGISTER_MODEL = gql`
  mutation RegisterModel($input: RegisterModelInput!) {
    registerModel(input: $input) {
      id
      status
    }
  }
`;

export const BIND_MODEL = gql`
  mutation BindModel($input: BindModelInput!) {
    bindModel(input: $input) {
      modelId
      objectType
      propertyId
    }
  }
`;

export const PREDICT = gql`
  mutation Predict($input: PredictInput!) {
    predict(input: $input)
  }
`;

export const GET_MODEL_BINDINGS = gql`
  query GetModelBindings($objectType: String) {
    modelBindings(objectType: $objectType) {
      modelId
      objectType
      propertyId
      boundAt
      config {
        inputProperties
      }
    }
  }
`;

export const UNBIND_MODEL = gql`
  mutation UnbindModel($objectType: String!, $propertyId: String!) {
    unbindModel(objectType: $objectType, propertyId: $propertyId)
  }
`;
