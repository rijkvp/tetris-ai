import i18n from 'sveltekit-i18n';
import type Config from 'sveltekit-i18n';

const config: Config = {
    translations: {
        en: {
            test: 'This is a test',
            weights: 'Weights',
            speed: 'Speed',
            scoreboard: 'Scoreboard',
            locale: {
                en: 'English',
                nl: 'Dutch',
            },
            nav: {
                levels: 'Levels',
                sandbox: 'Sandbox',
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
                completed: 'All goals completed!',
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
                    description: 'The number of cells that are cleared as a result of the move.',
                },
                cuml_wells: {
                    name: 'Cumulative wells',
                    description: 'The sum of the depths of all wells on the board. A well is a column with empty cells that are adjacent to filled cells.',
                },
            },
        },
        nl: {
            test: 'Dit is een test',
            weights: 'Gewichten',
            speed: 'Snelheid',
            scoreboard: 'Scorebord',
            locale: {
                en: 'Engels',
                nl: 'Nederlands',
            },
            nav: {
                levels: 'Levels',
                sandbox: 'Zandbak',
            },
            score: {
                score: 'Score',
                lines: 'Regels',
                level: 'Level',
                tetrises: 'Tetrissen',
            },
            goals: {
                goals: 'Doelen',
                completed: 'Alle doelen voltooid!',
            },
            control: {
                play: "Start",
                pause: "Pauze",
                reset: "Opnieuw",
                step: "Stap vooruit"
            },
            feature_control: {
                reset: "Herstel",
                randomize: "Willekeurig",
            },
            feature: {
                col_trans: {
                    name: 'Kolomtransities',
                    description: 'Het aantal keren dat twee hokjes naast elkaar in dezelfde kolom niet overeenkomen (de ene is gevuld en de andere is leeg).',
                },
                row_trans: {
                    name: 'Rijtransities',
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
                    description: 'Het aantal hokjes dat wordt gewist als gevolg van de zet.',
                },
                cuml_wells: {
                    name: 'Cumulatieve putten',
                    description: 'De som van de dieptes van alle putten op het bord. Een put is een kolom met lege hokjes die grenzen aan gevulde cellen.',
                },
            },
        },
    },
};

export const { t, locale, locales, loading, setLocale, loadTranslations } = new i18n(config);
