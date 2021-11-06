import {readFileSync} from 'fs';
import {parse} from 'node-html-parser';
import {model} from '../Model';

export const getParsedJSONLD = (filenames: string[]): StructuredData[] => {
    const result: StructuredData[] = [];

    filenames.forEach((filename) => {
        const file = readFileSync(filename, {encoding: model.charset, flag: 'r'});
        const root = parse(file);
        const scripts = root.querySelectorAll('script[type="application/ld+json"]');

        scripts.forEach((script) => {
            const object: {[key: string]: any} = {};

            object[filename] = JSON.parse(script.text);

            result.push(object);
        });
    });

    return result;
};
