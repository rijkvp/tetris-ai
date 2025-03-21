type MultiLang = {
    en: string;
    nl: string;
};
export type Level = {
    key: string;
    name: MultiLang;
    description: MultiLang;
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
        name: {
            en: "Level 1: 100 lines",
            nl: "Level 1: 100 rijen"
        },
        description: {
            en: "Try to clear hundred lines by adjusting the column transitions and row transitions.",
            nl: "Pas de waardes van de kenmerken zo aan zodat de AI honderd rijen weggespeelt."
        },
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
        name: {
            en: "Level 2: 4 times Tetris",
            nl: "Level 2: 4 keer Tetris"
        },
        description: {
            en: "Try to get four tetrises (clearing four rows at once) by using the landing height.",
            nl: "Probeer vier tetrissen te halen (vier rijen tegelijk wegspelen) door de landingshoogte aan te passen."
        },
        goals: {
            tetrises: 4
        },
        features: [
            "col_trans",
            "row_trans",
            "landing_height"
        ]
    },
    {
        key: "level3",
        name: {
            en: "Level 3: level 29",
            nl: "Level 3: level 29"
        },
        description: {
            en: "Try to reach level 29 and clear 200 lines by using 4 features.",
            nl: "Probeer level 29 te halen en 200 regels vrij te maken met behulp van 4 kenmerken."
        },
        goals: {
            lines: 200,
            level: 29
        },
        features: [
            "col_trans",
            "row_trans",
            "pits",
            "landing_height"
        ]
    }
];
