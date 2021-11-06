export const Log = {
    error(header: string, message: string): void {
        console.log('\x1b[31m%s\x1b[0m', `${header}: ${message}`);
    },
    warn(header: string, message: string): void {
        console.log('\x1b[33m%s\x1b[0m', `${header}: ${message}`);
    },
    debug(header: string, message: string): void {
        console.log('\x1b[37m%s\x1b[0m', `${header}: ${message}`);
    },
    info(header: string, message: string): void {
        console.log('\x1b[36m%s\x1b[0m', `${header}: ${message}`);
    },
};
