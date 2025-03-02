export type Level = {
    key: string;
    name: string;
    description: string;
    goals?: {
        lines: number;
        score: number;
        level: number;
        tetrises: number;
    },
    features?: string[]
};

export const levels: Level[] = [
    {
        key: "level1",
        name: "Level 1",
        description: "This is the first level",
        goals: {
            lines: 10,
            score: 0,
            level: 0,
            tetrises: 0
        },
        features: [
            "col_trans",
            "row_trans"
        ]
    },
    {
        key: "level2",
        name: "Level 2",
        description: "This is the second level",
        goals: {
            lines: 10,
            score: 1000,
            level: 1,
            tetrises: 1
        },
        features: [
            "col_trans",
            "row_trans"
        ]
    }
];
