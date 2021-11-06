// for execute from npm scripts or commandline.
import {configCheck} from './utils/ConfigCheck';

const globby = require('globby');
const yaml = require('yaml');
import {readFileSync} from 'fs';
import {Log} from './utils/LogSystem';
import {getParsedJSONLD} from './parser/HTMLParser';
import {getOptions} from './Commander';

(async () => {
    try {
        // set commander options.
        const argv = getOptions(process.argv);

        if (!argv.config) {
            Log.error('SDLint', 'please set path to .sdlintrc.yml');

            return;
        }

        const configString = readFileSync(argv.config, {encoding: 'utf-8'});
        const config: SDLintConfig = yaml.parse(configString);

        configCheck(config);

        const {include} = config;
        const exclude = config.exclude || [];
        const filePatterns = include.concat(exclude.map((item) => `!${item}`));

        // get vue entries from dir
        const entries: string[] = await globby(filePatterns, {
            expandDirectories: {
                extensions: ['html'],
            },
        });

        const jsonLDList = getParsedJSONLD(entries);

        // @TODO: lint all jsonLDLint
    } catch (err) {
        if (err instanceof Error) {
            Log.error('SDLint', err.message);
        }
    }
})();
