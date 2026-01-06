import React from 'react';
import {
    CubeIcon,
    LinkIcon,
    BoltIcon,
    Square3Stack3DIcon,
    ChartPieIcon
} from '@heroicons/react/24/outline';

interface HomeProps {
    onNavigate: (page: 'overview' | 'object-types' | 'link-types' | 'functions' | 'interfaces') => void;
}

export default function Home({ onNavigate }: HomeProps) {
    const sections = [
        {
            id: 'overview' as const,
            title: 'Ontology Overview',
            icon: ChartPieIcon,
            color: 'purple',
        },
        {
            id: 'object-types' as const,
            title: 'Object Types',
            icon: CubeIcon,
            color: 'blue',
        },
        {
            id: 'link-types' as const,
            title: 'Link Types',
            icon: LinkIcon,
            color: 'green',
        },
        {
            id: 'functions' as const,
            title: 'Functions',
            icon: BoltIcon,
            color: 'orange',
        },
        {
            id: 'interfaces' as const,
            title: 'Interfaces',
            icon: Square3Stack3DIcon,
            color: 'pink',
        },
    ];

    return (
        <div className="space-y-8">
            {/* Hero Section */}
            <div className="flex items-center justify-between">
                <div>
                    <h1 className="text-2xl font-bold text-dark-300">Ontology Management</h1>
                    <p className="text-gray-500 text-sm mt-1">Manage and explore your data model</p>
                </div>
                <div className="flex space-x-3">
                    <button className="px-3 py-1.5 bg-white border border-light-400 text-dark-300 text-sm font-medium rounded-sm shadow-sm hover:bg-light-100">
                        History
                    </button>
                    <button className="px-3 py-1.5 bg-foundry-core text-white text-sm font-medium rounded-sm shadow-sm hover:bg-foundry-hover">
                        New object type
                    </button>
                </div>
            </div>

            {/* Stats / Quick Info */}
            <div className="grid grid-cols-4 gap-4">
                <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm flex items-center justify-between">
                    <div>
                        <div className="text-2xl font-bold text-dark-300">3,312</div>
                        <div className="text-xs text-gray-500 uppercase tracking-wide font-medium mt-1">Object Types</div>
                    </div>
                    <CubeIcon className="w-8 h-8 text-light-400" />
                </div>
                <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm flex items-center justify-between">
                    <div>
                        <div className="text-2xl font-bold text-dark-300">4,555</div>
                        <div className="text-xs text-gray-500 uppercase tracking-wide font-medium mt-1">Link Types</div>
                    </div>
                    <LinkIcon className="w-8 h-8 text-light-400" />
                </div>
                <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm flex items-center justify-between">
                    <div>
                        <div className="text-2xl font-bold text-dark-300">5,423</div>
                        <div className="text-xs text-gray-500 uppercase tracking-wide font-medium mt-1">Functions</div>
                    </div>
                    <BoltIcon className="w-8 h-8 text-light-400" />
                </div>
                <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm flex items-center justify-between">
                    <div>
                        <div className="text-2xl font-bold text-dark-300">34</div>
                        <div className="text-xs text-gray-500 uppercase tracking-wide font-medium mt-1">Interfaces</div>
                    </div>
                    <Square3Stack3DIcon className="w-8 h-8 text-light-400" />
                </div>
            </div>

            {/* Recently Viewed Section */}
            <div className="space-y-4">
                <div className="flex items-center justify-between border-b border-light-300 pb-2">
                    <h2 className="text-lg font-semibold text-dark-300 flex items-center text-sm">
                        Recently viewed object types
                        <span className="ml-2 px-1.5 py-0.5 bg-light-300 text-dark-400 text-xs rounded-full">32</span>
                    </h2>
                    <button className="text-sm text-foundry-core hover:underline">See all →</button>
                </div>

                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                    {/* Card 1 */}
                    <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm hover:shadow-md transition-shadow group cursor-pointer">
                        <div className="flex items-start justify-between mb-3">
                            <div className="flex items-center">
                                <div className="w-8 h-8 bg-blue-100 rounded-sm flex items-center justify-center text-blue-600 mr-3">
                                    <CubeIcon className="w-5 h-5" />
                                </div>
                                <div>
                                    <h3 className="text-sm font-bold text-dark-300 group-hover:text-foundry-core">Campaign</h3>
                                    <p className="text-xs text-gray-500">16k objects</p>
                                </div>
                            </div>
                        </div>
                        <div className="flex gap-2 mb-3">
                            <span className="text-xs bg-light-200 text-dark-400 px-1.5 py-0.5 rounded border border-light-300">CRM <span className="text-gray-400 ml-1">46</span></span>
                            <span className="text-xs bg-light-200 text-dark-400 px-1.5 py-0.5 rounded border border-light-300">Marketing <span className="text-gray-400 ml-1">332</span></span>
                        </div>
                        <p className="text-xs text-gray-600 line-clamp-2">
                            A marketing campaign is a planned and organized effort to promote a specific component...
                        </p>
                        <div className="mt-4 pt-3 border-t border-light-200 flex justify-end">
                            <span className="text-xs text-gray-400 font-medium">9 dependents</span>
                        </div>
                    </div>

                    {/* Card 2 */}
                    <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm hover:shadow-md transition-shadow group cursor-pointer">
                        <div className="flex items-start justify-between mb-3">
                            <div className="flex items-center">
                                <div className="w-8 h-8 bg-indigo-100 rounded-sm flex items-center justify-center text-indigo-600 mr-3">
                                    <CubeIcon className="w-5 h-5" />
                                </div>
                                <div>
                                    <h3 className="text-sm font-bold text-dark-300 group-hover:text-foundry-core">Ticket</h3>
                                    <p className="text-xs text-gray-500">34k objects</p>
                                </div>
                            </div>
                        </div>
                        <div className="flex gap-2 mb-3">
                            <span className="text-xs bg-light-200 text-dark-400 px-1.5 py-0.5 rounded border border-light-300">Operations <span className="text-gray-400 ml-1">4</span></span>
                        </div>
                        <p className="text-xs text-gray-600 line-clamp-2">
                            A ticket is a term for an issue or work item that needs to be addressed or investigated.
                        </p>
                        <div className="mt-4 pt-3 border-t border-light-200 flex justify-end">
                            <span className="text-xs text-gray-400 font-medium">5 dependents</span>
                        </div>
                    </div>

                    {/* Card 3 */}
                    <div className="bg-white p-4 border border-light-300 rounded-sm shadow-sm hover:shadow-md transition-shadow group cursor-pointer">
                        <div className="flex items-start justify-between mb-3">
                            <div className="flex items-center">
                                <div className="w-8 h-8 bg-yellow-100 rounded-sm flex items-center justify-center text-yellow-600 mr-3">
                                    <CubeIcon className="w-5 h-5" />
                                </div>
                                <div>
                                    <h3 className="text-sm font-bold text-dark-300 group-hover:text-foundry-core">Workstation</h3>
                                    <p className="text-xs text-gray-500">2k objects</p>
                                </div>
                            </div>
                        </div>
                        <div className="flex gap-2 mb-3">
                            <span className="text-xs bg-light-200 text-dark-400 px-1.5 py-0.5 rounded border border-light-300">Equipment <span className="text-gray-400 ml-1">4</span></span>
                        </div>
                        <p className="text-xs text-gray-600 line-clamp-2">
                            Description text explaining how this is an object type for past and current...
                        </p>
                        <div className="mt-4 pt-3 border-t border-light-200 flex justify-end">
                            <span className="text-xs text-gray-400 font-medium">5 dependents</span>
                        </div>
                    </div>
                </div>
            </div>

            {/* Navigation Cards (Original) - kept but styled differently */}
            <div className="pt-6 border-t border-light-300">
                <h2 className="text-lg font-semibold text-dark-300 mb-4 text-sm">Quick Navigation</h2>
                <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                    {sections.map((section) => {
                        const Icon = section.icon;
                        return (
                            <button
                                key={section.id}
                                onClick={() => onNavigate(section.id)}
                                className="bg-light-100 items-center p-3 rounded-sm border border-light-300 hover:bg-white hover:border-foundry-core hover:shadow-sm transition-all text-left flex"
                            >
                                <Icon className={`w-5 h-5 text-${section.color}-600 mr-3`} />
                                <span className="text-sm font-medium text-dark-400">{section.title}</span>
                            </button>
                        );
                    })}
                </div>
            </div>
        </div>
    );
}
