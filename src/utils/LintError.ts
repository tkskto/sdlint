import {Log} from './LogSystem';

export const dumpErrorReport = (report: LintErrorReport): void => {
    Log.error('dump:', JSON.stringify(report, null, '    '));
};
