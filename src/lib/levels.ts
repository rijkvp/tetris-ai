type MultiLang = {
    en: string;
    nl: string;
};

export interface LevelInfo {
    name: MultiLang;
    summary: MultiLang;
    explanation?: MultiLang;
    sideText?: MultiLang;
};

/// This defines the ordering of the levels in the game.
export const LEVELS: string[] = [
    "play",
    "feat1",
    "feat2",
    "feat3",
    "feat4",
    "training",
    "sandbox",
];

export const LEVEL_INFO: { [key: string]: LevelInfo } = {
    "play": {
        name: {
            en: "Play Tetris",
            nl: "Speel Tetris"
        },
        summary: {
            en: "Play Tetris yourself to explore the game and its features.",
            nl: "Speel Tetris zelf om het spel te leren kennen."
        },
        explanation: {
            en: `<h2>Welcome!</h2>
        <p>
            This 'game' lets you explore how an artificial intelligence (AI)
            works that plays the game Tetris. You’ll learn how an AI plays
            Tetris and how the AI is 'trained' to play better.
        </p>

        <h3>New to Tetris? Here’s what you need to know:</h3>
        <ul>
            <li>
                <strong>Tetris</strong> is a classic video game where you fit pieces, called <em>tetrominos</em>, into a grid.
            </li>
            <li>
                <strong>The goal:</strong> Arrange the tetrominos to fill horizontal
                lines. When a line is full, it disappears, and you earn points.
            </li>
            <li>
                <strong>Earn more points:</strong> Clear multiple lines at once —especially four lines (called a <em>Tetris</em>)— to boost your score.
            </li>
            <li>
                <strong>Game over:</strong> If the tetrominos stack up to the top of the board, the game ends.
            </li>
        </ul>
        <p>
            First, let's start by playing Tetris yourself to get a feel for the
            game.
        </p>`,
            nl: `<h2>Welkom!</h2>
 <p>
 In dit 'spel' ontdek je hoe een kunstmatige intelligentie (AI) werkt die het spel Tetris speelt.
Je leert hoe de computer het spel Tetris kan spelen en hoe die wordt 'getraind'.
        </p>

        <h3>Ken je Tetris nog niet? Dit is wat je moet weten:</h3>
 <ul>
 <li>
 <strong>Tetris</strong> is een klassiek videospel waarin je blokjes, <em>tetrominos</em> genaamd, in een raster plaatst.
            </li>
 <li>
 <strong>Het doel:</strong> Plaats de tetromino's om horizontale rijen te vullen. Als een rij gevuld is, verdwijnt deze en verdien je punten.
            </li>
 <li>
 <strong>Verdien meer punten:</strong> Vul meerdere rijen tegelijk om je score te verhogen, de meeste punten krijg je door vier vier rijen tegelijk weg te spelen, dit wordt een <em>Tetris</em> genoemd.
            </li>
 <li>
 <strong>Game over:</strong> Als de tetromino de bovenkant van het scherm raken is het spel afgelopen.
            </li>
 </ul>
 <p>
Eerst ga je zelf Tetris spelen om een gevoel te krijgen voor het spel.</p>`,
        },
        sideText: {
            en: `<h2>Controls</h2>
        <ul>
            <li>
                <strong>Left/Right arrow keys</strong>: move the tetromino to
                the left/right
            </li>
            <li>Up arrow/Z: Rotate the tetromino</li>
            <li>Down arrow: Move the tetromino down (soft drop)</li>
            <li>Space: Drop the tetromino (hard drop)</li>
        </ul>`,
            nl: `<h2>Besturing</h2>
 <ul>
 <li>
 <strong>Pijltjes links/rechts</strong>: verplaats de tetromino naar links/rechts </li>
 <li><strong>Pijlje omhoog/Z</strong>: draai de tetromino</li>
 <li><strong>Pijlje omlaag</strong>: verplaats de tetromino naar beneden</li>
 <li><strong>Spatiebalk</strong>: laat de tetromino vallen</li>
 </ul>`
        }
    },
    "feat1": {
        name: {
            en: "Features",
            nl: "Kenmerken"
        },
        summary: {
            en: "Learn how to use features to make the AI play Tetris.",
            nl: "Leer hoe je kenmerken kunt gebruiken om de AI Tetris te laten spelen."
        },
        explanation: {
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
                The <em>goal</em> is to clear 20 lines by tweaking the weights of the features, good luck!
                </p>
            `,
            nl: `<p>Laten we nu de AI Tetris laten spelen! Computers zijn erg snel en kunnen kan gemakkelijk alle mogelijke plekken berekenen om de blokjes te plaatsen. Maar hij weet niet welke zetten goed zijn en welke slecht.</p>
<p>
Hier komen <strong>kenmerken</strong> om de hoek kijken. Kenmerken zijn aanwijzingen van stukjes informatie die de AI helpen om beslissingen te nemen. Door gewichten toe te kennen aan deze kenmerken laten we de AI weten welke soorten zetten goed zijn en welke juist slecht.</p>
<p>
In het volgende level begin je met twee kenmerken: <strong>Gaten</strong> en <strong>Landingshoogte</strong>. Je kunt de gewichten van de kenmerken aanpassen met schuifregelaars. Als je bijvoorbeeld denkt dat een kenmerken wenselijk is, kun je het gewicht verhogen zodat de AI deze prioriteert. Of als je denkt dat een kenmerk juist vermeden moet worden, kun je het gewicht verlagen. Het doel is om 20 lijnen weg te spelen door de gewichten van de kenmerken aan te passen, succes!</p>

`
        },
    },
    "feat2": {
        name: {
            en: "Features 2",
            nl: "Kolommen en rijen"
        },
        summary: {
            en: "Learn how to use features to make the AI play Tetris.",
            nl: "Leer hoe kolom- en rijovergangen werken."
        },
        explanation: {
            en: "Try to clear hundred lines by adjusting the column transitions and row transitions.",
            nl: `<p>In dit level zijn er twee andere kenmerken: <strong>Kolomovergangen</strong> en <strong>Rijovergangen</strong>.
Probeer met deze kenmerken honderd rijen weg te spelen. Voor uitleg over de kenmerken kun je de <strong>[?]</strong> knop gebruiken.</p>`
        },
    },
    "feat3": {
        name: {
            en: "Tetris",
            nl: "Tetris"
        },
        summary: {
            en: "Learn how to use features to make the AI play Tetris.",
            nl: "Leer de AI om een Tetris te maken."
        },
        explanation: {
            en: "Try to get a tetris (clearing four rows at once) by using the cleared cells and wells features.",
            nl: `<p>Probeer een <strong>Tetris</strong> te halen door vier rijen tegelijk wegspelen.
Doe dit met behulp van de <strong>Gewiste hokjes</strong> en <strong>Putten</strong> kenmerken.
De gewichten van twee andere kenmerken zijn al gegeven om je op weg te helpen. Deze kun je niet meer aanpassen.
            </p>`
        },
    },
    "feat4": {
        name: {
            en: "Level 29",
            nl: "Level 29"
        },
        summary: {
            en: "Reach level 29 by using 6 features.",
            nl: "Behaal level 29 halen met 6 kenmerken."
        },
        explanation: {
            en: "Try to reach level 29.",
            nl: `<p>
Nu gaan we alle kenmerken combineren. In Tetris is het moeilijkste level 29. Probeer dit level te halen door de weer de gewichten van de <strong>Gewiste hokjes</strong> en <strong>Putten</strong> aan te passen. De 4 andere kenmerken zijn al gegeven.</p>`
        },
    },
    "training": {
        name: {
            en: "Training",
            nl: "Training"
        },
        summary: {
            en: "Explore how the AI is trained to play Tetris.",
            nl: "Leer hoe de AI wordt getraind om Tetris te spelen."
        },
        explanation: {
            en: "In this level you can see how to AI is trained.",
            nl: `<p>In de praktijk is het (zoals je misschien gemerkt hebt) niet praktisch om alle kenmerken handmatig in te stellen. AI modellen kunnen wel duizenden kenmerken hebben. In plaats daarvan worden AIs 'getraind' door een algoritme. In dit 'level' kun je zien hoe de Tetris AI wordt getraind door een algoritme. In de visualisatie zie je steeds de beste versie van de AI van de vorige generatie. Je ziet dat die steeds beter wordt in het spelen van Tetris.

<h3>Hoe werkt het trainingsalgoritme?</h3>

<p>
Het proces begint met een generatie van 100 Tetris-modellen, die elk willekeurige gewichten krijgen. Nadat elk model een potje Tetris heeft gespeeld, worden de 10 beste modellen gekozen op basis van hun score. Deze topmodellen vormen samen de basis voor de volgende generatie.
</p>

<p>
Het gemiddelde van de gewichten van de topmodellen wordt genoemen en dat gemiddelde wordt vervolgens een beetje aangepast met kleine, willekeurige veranderingen. Zo ontstaat een nieuwe generatie modellen, die hopelijk nog beter speelt. Dit process wordt net zolang herhaald totdat alle modellen op dezelfde kenmerken uitkomen.
</p>`
        },
    },
    "sandbox": {
        name: { en: "Sandbox", nl: "Sandbox-modus" },
        summary: {
            en: "Play around with all the features.",
            nl: "Speel met alle kenmerken.",
        },
        sideText: {
            en: "Here you can play around with all the features.",
            nl: "Hier kun je met alle kenmerken spelen.",
        }
    },
};


export interface LevelConfig {
    goals: Goals;
    features?: [string, number][];
    lockedFeatures?: string[];
};

export interface Goals {
    lines?: number;
    score?: number;
    level?: number;
    tetrises?: number;
};

export const LEVEL_CONFIG: { [key: string]: LevelConfig } = {
    "feat1": {
        goals: {
            lines: 20,
        },
        features: [
            ["pits", 0],
            ["landing_height", 0]
        ]
    },
    "feat2": {
        goals: {
            lines: 100,
        },
        features: [
            ["col_trans", 0],
            ["row_trans", 0]
        ]
    },
    "feat3": {
        goals: {
            tetrises: 1
        },
        features: [
            ["pits", -10],
            ["landing_height", -3],
            ["eroded_cells", 0],
            ["cuml_wells", 0],
        ],
        lockedFeatures: [
            "pits",
            "landing_height"
        ]
    },
    "feat4": {
        goals: {
            level: 29
        },
        features: [
            ["col_trans", -8.5],
            ["row_trans", -2.5],
            ["pits", -10],
            ["landing_height", -5],
            ["eroded_cells", 0],
            ["cuml_wells", 0]
        ],
        lockedFeatures: [
            "col_trans",
            "row_trans",
            "pits",
            "landing_height"
        ],
    }
};
