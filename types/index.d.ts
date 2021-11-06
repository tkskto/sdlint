interface StructuredData {
    [filename: string]: {
        '@context': string;
        '@type': string;
    }
}

type SDLintConfig = {
    vocabularies: string;
    syntax: string;
    charset: BufferEncoding;
    include: string[];
    exclude: string[];
}

type SDLintMessageLevel = 'error' | 'warn';

type SDLintMessage = {
    level: SDLintMessageLevel;
    message: string
}

interface LintError {
    filename: string;
    messages: SDLintMessage[];
}

interface LintErrorReport {
    errors: LintError[];
    summary: {
        countError: number,
        countWarn: number,
    };
}
