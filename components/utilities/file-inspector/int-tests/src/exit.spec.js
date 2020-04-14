const helper = require('./helpers/test.helper');
const TIMEOUT = 20000;

describe('/exit', () => {
    beforeAll(async () => {
        await helper.waitForSpinup();
    }, TIMEOUT);

    it('should stop the server', async () => {
        const uptime = await helper.getUptime();
        const response = await helper.callExit();
        expect(response.status).toEqual(202);

        await helper.waitForRestart(uptime);
    }, TIMEOUT);
});
