const helper = require('./helpers/test.helper');
const TIMEOUT = 20000;

const DATA_DIR = process.env['TARGET_DIR'];

describe('/list', () => {
    beforeAll(async () => {
        await helper.waitForSpinup();
    }, TIMEOUT);

    it('should return a list of files', async () => {
        const { status, data } = await helper.listContents(DATA_DIR);

        expect(status).toEqual(200);
        expect(data).toEqual({ dirs: [ 'dir1', 'dir2' ] });
    });
});