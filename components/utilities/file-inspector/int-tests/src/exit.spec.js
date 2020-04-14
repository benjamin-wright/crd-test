const helper = require('./helpers/test.helper');
const TIMEOUT = 20000;

describe('/exit', () => {
    beforeAll(async () => {
        await helper.waitForSpinup();
    }, TIMEOUT);

    it('should stop the server', () => {
    });
});