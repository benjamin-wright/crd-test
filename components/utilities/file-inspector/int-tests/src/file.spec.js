const helper = require('./helpers/test.helper');
const TIMEOUT = 20000;

describe('/file', () => {
    beforeAll(async () => {
        await helper.waitForSpinup();
    }, TIMEOUT);

    it('should fetch the named file', async () => {
        const { status, data } = await helper.getFile('file1.yaml');

        expect(status).toEqual(200);
        expect(data).toEqual('param1: value1\nparam2: value2');
    });

    it('should fetch the other named file', async () => {
        const { status, data } = await helper.getFile('file2.yaml');

        expect(status).toEqual(200);
        expect(data).toEqual('param3: value3\nparam4: value4');
    });

    it('should fetch a file from a subdirectory', async () => {
        const { status, data } = await helper.getFile('dir1/file3.yaml');

        expect(status).toEqual(200);
        expect(data).toEqual('param5: value5\nparam6: value6');
    });

    it('should not fetch a file which does not exist', async () => {
        await expect(helper.getFile('file-missing.yaml')).rejects.toEqual(new Error("Error making request: 404"));
    });
});