import {program} from 'commander';

export const getOptions = (_argv: string[]): {
    config: string,
    format: 'codeframe',
} => {
    program.option('-c, --config [string]', 'path to .sdlintrc');
    program.option('-f, --format [string]', 'formatter');
    program.parse(_argv);

    return program.opts();
};
