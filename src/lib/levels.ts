type MultiLang = {
    en: string;
    nl: string;
};

export interface Goals {
    lines?: number;
    score?: number;
    level?: number;
    tetrises?: number;
};

export interface Level {
    key: string;
    name: MultiLang;
    description: MultiLang;
    goals: Goals;
    features?: string[]
};

export const NAVIGATION: string[] = [
    "intro",
    "2",
    "3",
    "4",
    "5",
    "sandbox",
];


export const levels: Level[] = [
    {
        key: "2",
        name: {
            en: "Features",
            nl: "Kenmerken"
        },
        description: {
            en: `
                <p>
                Now let's make the AI play Tetris!
                Computers are really fast and the AI can easiliy calculate all possible splots to place the tetronomino.
                However, it does not know what moves are good, and which are bad.
                </p>
                <p>
                This is where <em>features</em> come in. Features are pieces of information that help the AI make decisions. Think of them as clues or characteristics that describe something.
                We assign weights to these features to let the AI know which types of moves are good and which tyes are bad.
                </p>
                <p>
                In the following level you will start with two features: <em>Holes</em> and <em>Landing Height</em>.
                You can adjust the <em>weights</em> of these features using the sliders. For example, if you think a feature is desired, you can increase its weight to make the AI prioritize it. Or, if you think a feature should be avoided, you can decrease its weight.
                Your <em>goal</em> is to clear 20 lines by tweaking the weights of the features!
                </p>
            `,
            nl: "TODO"
        },
        goals: {
            lines: 20,
        },
        features: [
            "pits",
            "landing_height"
        ]
    },
    {
        key: "3",
        name: {
            en: "Features 2",
            nl: "Kenmerken 2"
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
        key: "4",
        name: {
            en: "4 times Tetris",
            nl: "4 keer Tetris"
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
        key: "5",
        name: {
            en: "Level 29",
            nl: "Level 29"
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
