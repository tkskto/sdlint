import {Log} from './LogSystem';

export const configCheck = (config: SDLintConfig): void => {
    let result = true;
    const {charset, syntax, vocabularies, include} = config;

    if (!charset) {
        Log.error('SDLint', '.sdlintrc.yml must have charset property.');
        result = false;
    }

    if (!syntax) {
        Log.error('SDLint', '.sdlintrc.yml must have syntax property.');
        result = false;
    }

    if (!vocabularies) {
        Log.error('SDLint', '.sdlintrc.yml must have vocabularies property.');
        result = false;
    } else if (vocabularies === 'http://www.data-vocabulary.org/') {
        Log.warn('SDLint', 'data-vocabulary.org is deprecated. See https://developers.google.com/search/blog/2020/01/data-vocabulary For more information.');
    }

    if (!include) {
        Log.error('SDLint', '.sdlintrc.yml must have include property.');
        result = false;
    }

    if (!result) {
        throw new Error('.sdlintrc.yml have some problem.');
    }
};
