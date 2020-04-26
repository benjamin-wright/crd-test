const sidecar = require('./helpers/sidecar');

const START_TIMEOUT = 15000;

describe('version-sidecar', () => {
    beforeAll(async () => {
        await sidecar.init();
    }, START_TIMEOUT);

    it('should fail if the version file is missing', async () => {
        const test = await sidecar.runTest('test-1');
        const status = await test.exitStatus();

        expect(status).toEqual("Failed");
    });

    it('should fail if the version file is empty', async () => {
        const test = await sidecar.runTest('test-2', '');
        const status = await test.exitStatus();

        expect(status).toEqual("Failed");
    });

    it('should pass if the version file is present', async () => {
        const test = await sidecar.runTest('test-3', 'abcdefg');
        const status = await test.exitStatus();

        expect(status).toEqual("Succeeded");
    });
});