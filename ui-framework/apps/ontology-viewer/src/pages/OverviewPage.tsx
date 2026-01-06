import React from 'react';
import { useQuery, gql } from '@apollo/client';
import { useOntology } from '@ontology/core';

const GET_OVERVIEW = gql`
  query GetOverview {
    getObjectTypes {
      id
      displayName
    }
    getFunctions {
      id
      displayName
    }
    getInterfaces {
      id
      displayName
      implementers {
        objectType
        count
      }
    }
  }
`;

export default function OverviewPage() {
    const { client } = useOntology();
    const { data, loading, error } = useQuery(GET_OVERVIEW, { client });

    if (loading) return <div className="text-center py-12">Loading ontology overview...</div>;
    if (error) return <div className="text-red-500 p-4">Error: {error.message}</div>;

    const objectTypes = data?.getObjectTypes || [];
    const functions = data?.getFunctions || [];
    const interfaces = data?.getInterfaces || [];
    const totalImplementers = interfaces.reduce(
        (sum: number, i: any) => sum + (i.implementers?.length || 0),
        0
    );

    const stats = [
        { label: 'Object Types', value: objectTypes.length, color: 'blue' },
        { label: 'Functions', value: functions.length, color: 'green' },
        { label: 'Interfaces', value: interfaces.length, color: 'purple' },
        { label: 'Interface Implementations', value: totalImplementers, color: 'orange' },
    ];

    return (
        <div className="px-4 py-6 sm:px-0">
            <h1 className="text-2xl font-bold mb-6">Ontology Overview</h1>

            {/* Stats Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
                {stats.map((stat) => (
                    <div key={stat.label} className="bg-white p-6 rounded-lg shadow-sm border">
                        <div className={`text-3xl font-bold text-${stat.color}-600`}>{stat.value}</div>
                        <div className="text-gray-500 text-sm mt-1">{stat.label}</div>
                    </div>
                ))}
            </div>

            {/* Object Types Summary */}
            <div className="bg-white rounded-lg shadow-sm border p-6 mb-6">
                <h2 className="text-lg font-semibold mb-4">Object Types</h2>
                <div className="flex flex-wrap gap-2">
                    {objectTypes.map((ot: any) => (
                        <span key={ot.id} className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">
                            {ot.displayName || ot.id}
                        </span>
                    ))}
                    {objectTypes.length === 0 && (
                        <span className="text-gray-500 text-sm">No object types defined</span>
                    )}
                </div>
            </div>

            {/* Functions Summary */}
            <div className="bg-white rounded-lg shadow-sm border p-6 mb-6">
                <h2 className="text-lg font-semibold mb-4">Functions</h2>
                <div className="flex flex-wrap gap-2">
                    {functions.map((f: any) => (
                        <span key={f.id} className="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm">
                            {f.displayName || f.id}
                        </span>
                    ))}
                    {functions.length === 0 && (
                        <span className="text-gray-500 text-sm">No functions defined</span>
                    )}
                </div>
            </div>

            {/* Interfaces Summary */}
            <div className="bg-white rounded-lg shadow-sm border p-6">
                <h2 className="text-lg font-semibold mb-4">Interfaces</h2>
                <div className="flex flex-wrap gap-2">
                    {interfaces.map((i: any) => (
                        <span key={i.id} className="px-3 py-1 bg-purple-100 text-purple-800 rounded-full text-sm">
                            {i.displayName || i.id} ({i.implementers?.length || 0} implementers)
                        </span>
                    ))}
                    {interfaces.length === 0 && (
                        <span className="text-gray-500 text-sm">No interfaces defined</span>
                    )}
                </div>
            </div>
        </div>
    );
}
