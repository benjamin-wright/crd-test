const helper = require('./helpers/test.helper');
const TIMEOUT = 20000;

const DATA_DIR = process.env['TARGET_DIR'];

describe('/list', () => {
    beforeAll(async () => {
        await helper.waitForSpinup();
    }, TIMEOUT);

    it('should return a list of files', async () => {
        const { status, data } = await helper.listContents();
        const expectedFiles = [ 'file1.yaml', 'file2.yaml', 'dir1/file3.yaml', 'dir2/file4.yaml' ].sort();
        data.files.sort();

        expect(status).toEqual(200);
        expect(data).toEqual({ path: DATA_DIR, files: expectedFiles });
    });
});