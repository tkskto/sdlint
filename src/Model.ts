class Model {
    private _vocabularies = 'https://schema.org';

    private _syntax = 'JSON-LD';

    private _charset: BufferEncoding = 'utf-8';

    get syntax(): string {
        return this._syntax;
    }

    set syntax(value: string) {
        this._syntax = value;
    }

    get vocabularies(): string {
        return this._vocabularies;
    }

    set vocabularies(value: string) {
        this._vocabularies = value;
    }

    get charset(): BufferEncoding {
        return this._charset;
    }

    set charset(value: BufferEncoding) {
        this._charset = value;
    }
}

export const model = new Model();
