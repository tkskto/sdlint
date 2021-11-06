export const setMessageLevel = (level: SDLintMessageLevel, message: string): SDLintMessage => ({
    level,
    message,
});

export const dontHaveTypeProperty = (): SDLintMessage => setMessageLevel('error', 'each data must have type property.');

export const dontMatchTypeProperty = (type: string, expected: string, received: string): SDLintMessage => setMessageLevel('error', `type of ${type} should be ${expected}, not ${received}.`);

export const dontHaveRequiredProperty = (type: string, property: string): SDLintMessage => setMessageLevel('error', `type of ${type} must have ${property} property.`);

export const dontHaveRecommendedProperty = (type: string, property: string): SDLintMessage => setMessageLevel('warn', `type of ${type} should have ${property} property.`);

export const shouldNotBeEmpty = (what: string): SDLintMessage => setMessageLevel('error', `${what} should not be empty.`);
