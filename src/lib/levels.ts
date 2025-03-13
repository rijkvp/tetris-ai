export type Level = {
    key: string;
    name: string;
    description: string;
    goals?: {
        lines?: number;
        score?: number;
        level?: number;
        tetrises?: number;
    },
    features?: string[]
};

export const levels: Level[] = [
    {
        key: "level1",
        name: "Level 1",
        description: "Try to clear 100 lines using the column transitions and row transitions features",
        goals: {
            lines: 100,
        },
        features: [
            "col_trans",
            "row_trans"
        ]
    },
    {
        key: "level2",
        name: "Level 2",
        description: "Try to get 4 tetrises using the extra feature landing height",
        goals: {
            tetrises: 4
        },
        features: [
            "col_trans",
            "row_trans",
            "landing_height"
        ]
    }
];
