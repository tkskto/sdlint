# SDLint

SDLint is a tool for find problems in your structured data with [Ajv](https://ajv.js.org/).

Support only [schema.org](https://schema.org/) vocabularies and [JSON-LD](https://json-ld.org/) syntax.

Almost rules follow [Google Search Central](https://developers.google.com/search/docs) guideline.

## Why

Usually we write structured data by your hand. We don't have a process to check these structured data is correct.

## not goal

*   check consistency of the structured data and actual contents.

## installation and usage

**not publish yet. But planning to Following steps.**

You can install sdlint using npm:

```
npm install sdlint
```

Then put command on npm scripts.

```
"scripts": {
  "sdlint": "sdlint -c .sdlintrc.yml"
}
```

Or you can use npx:

```
npx sdlint -c .sdlintrc.yml
```

### option

WIP

There are few options to use sdlint better.

*   considering formatting result

### API

WIP

*   considering use sdlint as ESM

## configuration

SDLint use `.sdlintrc.yml` for find target files. So you must put `.sdlintrc.yml` into your project.

```yaml
vocabularies: 'https://schema.org/'
syntax: 'JSON-LD'
charset: 'utf-8'
include:
  - 'test/**/**.html'
exclude:
```

|option|description|
|---|---|
|vocabularies|reference of vocabularies. currently, support only [`schema.org`](https://schema.org/)|
|syntax|structured data format. currently, support only `JSON-LD`.|
|charset|HTML charset. (default 'utf-8')|
|include|target files. (you can use glob pattern)|
|exclude|not target files. (you can use glob pattern)|

## Support Types.

SDLint do not support all vocabularies yet. following rules.

|type|spec (URL)|
|----|----|
|BreadcrumbList|https://schema.org/BreadcrumbList|
|JobPosting|https://schema.org/JobPosting|
