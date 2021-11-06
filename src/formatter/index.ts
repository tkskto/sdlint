import {iFormatter} from './iFormatter';
import {codeframeFormatter} from './CodeframeFormatter';

export class Formatter {
    private _formatter: iFormatter;

    constructor(formatterType: 'codeframe') {
        switch (formatterType) {
        case 'codeframe':
            this._formatter = codeframeFormatter;
            break;
        default:
            this._formatter = codeframeFormatter;
            break;
        }
    }

    public report(errorReport: LintErrorReport): void {
        this._formatter.report(errorReport);
    }
}
