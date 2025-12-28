import React, { useState } from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

const GET_AVAILABLE_YEARS = gql`
  query GetAvailableYears($objectType: String!) {
    getAvailableYears(objectType: $objectType)
  }
`;

const TEMPORAL_QUERY = gql`
  query TemporalQuery(
    $objectType: String!
    $year: Int
    $yearRangeStart: Int
    $yearRangeEnd: Int
    $asOfDate: String
  ) {
    temporalQuery(
      objectType: $objectType
      year: $year
      yearRangeStart: $yearRangeStart
      yearRangeEnd: $yearRangeEnd
      asOfDate: $asOfDate
    ) {
      objectType
      objectId
      title
      properties
    }
  }
`;

export interface TimeSliderProps {
  objectType: string;
  onYearChange?: (year: number, objects: any[]) => void;
}

export function TimeSlider({ objectType, onYearChange }: TimeSliderProps) {
  const { client } = useOntology();
  const [selectedYear, setSelectedYear] = useState<number | null>(null);

  const { data: yearsData } = useQuery(GET_AVAILABLE_YEARS, {
    client,
    variables: { objectType },
  });

  const { data: temporalData } = useQuery(TEMPORAL_QUERY, {
    client,
    variables: {
      objectType,
      year: selectedYear,
    },
    skip: selectedYear === null,
  });

  const years = yearsData?.getAvailableYears || [];
  const objects = temporalData?.temporalQuery || [];

  React.useEffect(() => {
    if (selectedYear !== null && objects.length > 0) {
      onYearChange?.(selectedYear, objects);
    }
  }, [selectedYear, objects, onYearChange]);

  if (years.length === 0) {
    return <div>No years available for {objectType}</div>;
  }

  const minYear = Math.min(...years);
  const maxYear = Math.max(...years);

  return (
    <div className="time-slider p-4 border rounded">
      <label className="block text-sm font-medium mb-2">
        Year: {selectedYear ?? 'Select a year'}
      </label>
      <input
        type="range"
        min={minYear}
        max={maxYear}
        value={selectedYear ?? minYear}
        onChange={(e) => setSelectedYear(parseInt(e.target.value))}
        className="w-full"
      />
      <div className="flex justify-between text-xs text-gray-500 mt-1">
        <span>{minYear}</span>
        <span>{maxYear}</span>
      </div>
      {selectedYear && (
        <div className="mt-2 text-sm">
          Found {objects.length} objects for year {selectedYear}
        </div>
      )}
    </div>
  );
}



