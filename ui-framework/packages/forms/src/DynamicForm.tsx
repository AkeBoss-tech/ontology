import React from 'react';
import { PropertyEditor, PropertyDefinition } from '@ontology/core';

export interface DynamicFormProps {
  properties: PropertyDefinition[];
  initialValues?: Record<string, any>;
  onSubmit: (values: Record<string, any>) => void;
  submitLabel?: string;
}

export function DynamicForm({
  properties,
  initialValues = {},
  onSubmit,
  submitLabel = 'Submit',
}: DynamicFormProps) {
  const [values, setValues] = React.useState<Record<string, any>>(initialValues);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSubmit(values);
  };

  return (
    <form onSubmit={handleSubmit} className="dynamic-form space-y-4">
      <PropertyEditor properties={properties} values={values} onChange={setValues} />
      <button
        type="submit"
        className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
      >
        {submitLabel}
      </button>
    </form>
  );
}




