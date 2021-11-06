import {iFormatter} from './iFormatter';

class CodeframeFormatter implements iFormatter {
    public report(errorReport: LintErrorReport) {
        console.log(errorReport);
    }
}

export const codeframeFormatter = new CodeframeFormatter();
