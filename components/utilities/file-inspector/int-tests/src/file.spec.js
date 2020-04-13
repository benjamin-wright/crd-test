const helper = require('./helpers/test.helper');
const TIMEOUT = 20000;

describe('/file', () => {
    beforeAll(async () => {
        await helper.waitForSpinup();
    }, TIMEOUT);

    it('should fetch the named file', async () => {
        const { status, data } = await helper.getFile('file1.yaml');

        expect(status).toEqual(200);
        expect(data).toEqual({});
    });
});