import type { Writable } from 'svelte/store';
import i18n from 'sveltekit-i18n';

const config = {
    translations: {
        en: {
            general: {
                close: 'Close',
                example: 'Example',
                features: 'Weights',
                speed: 'Speed',
                scoreboard: 'Scoreboard',
            },
            level_select: {
                title: 'Select level',
                description: 'Select a level to play.',
            },
            locale: {
                en: 'English',
                nl: 'Dutch',
            },
            nav: {
                levels: 'Levels',
                sandbox: 'Sandbox',
                guide: 'Guide',
            },
            guide: {
                title: 'Guide',
                text: `This tool lets you control an AI playing Tetris, a classic game. There are different levels with different goals that you can play so you can learn how the AI works. Sandbox mode lets you experiment with all the features. Have fun!

<h2>How do I control the AI?</h2>

At the top of the Tetris board, there are buttons that allow you to play, pause, reset, or step through the game. You can also adjust the speed of the game and the weights of the features. On the left, you can see the stats of the current game played by the AI. It shows the score (the more lines cleared, the higher the score), the number of lines cleared, the current level (every 10 lines is a level), and the number of tetrises you have made (clearing 4 rows at once). The goals show the objectives you need to complete to finish the level.

<h2>How does the AI work?</h2>

The AI calculates all possible positions and rotations where it can be placed for every tetromino it has to play. Then, it evaluates them using a set of features. The AI then chooses the move with the highest score. You can adjust the importance of different features using the sliders to meet the goals. The weights of these features determine how important they are in the evaluation. For example, if you think a feature is very important, you can increase its weight to make the AI prioritize it. Or, if you think a feature should be avoided, you can decrease its weight.`,
            },
            score: {
                score: 'Score',
                lines: 'Lines',
                level: 'Level',
                tetrises: 'Tetrises',
            },
            controls: {
                play: "Play",
                pause: "Pause",
                reset: "Reset",
                step: "Step",
            },
            goals: {
                goals: 'Goals',
                complete: 'Level complete! You reached all goals.',
                info: 'Complete all goals to finish the level.',
            },
            feature_control: {
                reset: "Reset",
                randomize: "Randomize",
            },
            feature: {
                col_trans: {
                    name: 'Column transitions',
                    description: 'The number of times that two cells next to each other in the same column don\'t match (one is filled and the other is empty).',
                },
                row_trans: {
                    name: 'Row transitions',
                    description: 'The number of times that two cells next to each other in the same row don\'t match (one is filled and the other is empty).',
                },
                pits: {
                    name: 'Holes',
                    description: 'The number of empty cells that have a filled cell above them.',
                },
                landing_height: {
                    name: 'Landing height',
                    description: 'The height at which the tetromino is placed.',
                },
                eroded_cells: {
                    name: 'Cleared cells',
                    description: 'The number of cells that are cleared from the tetromino itself as a result of the move.',
                },
                cuml_wells: {
                    name: 'Cumulative wells',
                    description: 'The sum of the depths of all wells on the board. A well is a column with empty cells that are adjacent to filled cells.',
                },
            },
        },
        nl: {
            general: {
                close: 'Sluiten',
                example: 'Voorbeeld',
                features: 'Kenmerken',
                speed: 'Snelheid',
                scoreboard: 'Scorebord',
            },
            level_select: {
                title: 'Level selecteren',
                description: 'Kies een level om te spelen.',
            },
            locale: {
                en: 'Engels',
                nl: 'Nederlands',
            },
            nav: {
                levels: 'Levels',
                sandbox: 'Sandbox-modus',
                guide: 'Uitleg',
            },
            guide: {
                title: 'Uitleg',
                text: `Met deze tool kun je een AI besturen die Tetris speelt, een klassieke videogame. Er zijn verschillende niveaus met verschillende doelen die je kunt spelen zodat je kunt leren hoe de AI werkt. In de Sandbox-modus kun je experimenteren met alle functies. Veel plezier!

<h2>Wat is Tetris?</h2>

Tetris is een klassieke videogame waarin je puzelstukjes, die tetromino's worden genoemd, moet plaatsen om rijen te vullen. Als je een rij vult, verdwijnt deze en krijg je punten. Het doel is om zoveel mogelijk rijen te vullen en zo lang mogelijk te overleven. De meeste punten krijg je door een tetris te maken, door 4 rijen tegelijk weg te spelen. Als de tetromino's tot aan de bovenkant van het bord komen, is het spel voorbij.

<h2>Hoe bestuur ik de AI?</h2>

Bovenaan het Tetris-bord vind je knoppen waarmee je de AI kunt starten en pauzeren of het bord herstellen. Je kunt ook de snelheid van het spel en de waardes van de kenmarken aanpassen. Links zie je de statistieken van het huidige spel dat door de AI wordt gespeeld. Je ziet de score (hoe meer rijen je hebt weggespeeld, hoe hoger de score), het aantal weggespeelde rijen, het huidige level (elke 10 rijen is een level) en het aantal tetrissen dat de AI heeft gemaakt (4 rijen tegelijk weggespeeld). De doelen tonen aan wat je moet doen om het level te voltooien. Om een uitleg te krijgen over een kernmerk, druk op de [?] knop.

<h2>Hoe werkt de AI?</h2>

De AI berekent alle mogelijke posities en rotaties waar een tetromino geplaatst kan worden. Daarna telt hij aan de hand van een aantal kenmerken de score op die elke zet geeft. De AI kiest dan de zet met de hoogste score. Je kunt het belang van verschillende kenmerken aanpassen met de schuifregelaars om de doelen te bereiken. De waardes van deze kenmerken bepalen hoe belangrijk ze zijn in de evaluatie. Als je bijvoorbeeld denkt dat een kenmerk erg belangrijk is, kun je het waarde ervan verhogen zodat de AI er prioriteit aan geeft. Of als je denkt dat een kenmerk moet worden vermeden, kun je het waarde verlagen.`,
            },
            score: {
                score: 'Punten',
                lines: 'Rijen',
                level: 'Level',
                tetrises: 'Tetrissen',
            },
            controls: {
                play: "Start",
                pause: "Pauze",
                reset: "Herstel",
                step: "Stap",
            },
            goals: {
                goals: 'Doelen',
                complete: 'Level voltooid! Je hebt alle doelen bereikt.',
                info: 'Behaal alle doelen om dit level te voltooien.',
            },
            feature_control: {
                reset: "Herstel",
                randomize: "Willekeurig",
            },
            feature: {
                col_trans: {
                    name: 'Kolomovergangen',
                    description: 'Het aantal keren dat twee hokjes boven elkaar in dezelfde kolom niet overeenkomen (de ene is gevuld en de andere is leeg).',
                },
                row_trans: {
                    name: 'Rijovergangen',
                    description: 'Het aantal keren dat twee hokjes naast elkaar in dezelfde rij niet overeenkomen (de ene is gevuld en de andere is leeg).',
                },
                pits: {
                    name: 'Gaten',
                    description: 'Het aantal lege cellen dat een gevulde cel erboven heeft.',
                },
                landing_height: {
                    name: 'Landingshoogte',
                    description: 'De hoogte waarop de tetromino wordt geplaatst.',
                },
                eroded_cells: {
                    name: 'Gewiste hokjes',
                    description: 'Het aantal hokjes dat wordt gewist van de tetromino zelf als gevolg van de zet.',
                },
                cuml_wells: {
                    name: 'Cumulatieve putten',
                    description: 'De som van de dieptes van alle putten op het bord. Een put is een kolom met lege hokjes die grenzen aan gevulde cellen.',
                },
            },
        },
    },
};

const stores = new i18n(config);
export const { t, locales, loading, loadTranslations } = stores;
export const locale = stores.locale as unknown as Writable<"en" | "nl">;
export const setLocale = stores.setLocale as (locale: "en" | "nl") => void;
