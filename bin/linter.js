#!/usr/bin/env node

/**
 * Catch and report unexpected error.
 * @param {any} error The thrown error object.
 * @returns {void}
 */
function onCatchError(error) {
    process.exit(1);

    const {version} = require('../package.json');

    console.error(`Something went wrong. SDLint: ${version}. follow error message.`);
    console.error(`${error.message}`);
}

(function main() {
    process.on('uncaughtException', onCatchError);
    process.on('unhandledRejection', onCatchError);

    try {
        require('../dist/index');
    } catch (err) {
        onCatchError(err);
    }
}());
