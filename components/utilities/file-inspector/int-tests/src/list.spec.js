const helper = require('./helpers/test.helper');
const TIMEOUT = 20000;

describe('/list', () => {
    beforeAll(async () => {
        await helper.waitForSpinup();
    }, TIMEOUT);

    it('should work', () => {
        console.info('okay');
    });
});