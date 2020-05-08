const sidecar = require('./helpers/sidecar');
const api = require('./helpers/api');

const START_TIMEOUT = 15000;
const TEST_TIMEOUT = 10000;

describe('version-sidecar', () => {
    beforeAll(async () => {
        await sidecar.init();
        await api.init();
    }, START_TIMEOUT);

    it('should fail if the version file is missing', async () => {
        const test = await sidecar.runTest('test-1');
        const status = await test.exitStatus();

        expect(status).toEqual("Failed");
    }, TEST_TIMEOUT);

    it('should fail if the version file is empty', async () => {
        const test = await sidecar.runTest('test-2', '');
        const status = await test.exitStatus();

        expect(status).toEqual("Failed");
    }, TEST_TIMEOUT);

    it('should pass if the version file is present', async () => {
        await api.create('test-3', { resource: 'res', pipeline: 'pipeline', version: 'abcd' });
        const test = await sidecar.runTest('test-3', 'abcdefg', 'resource', 'pipeline');
        const status = await test.exitStatus();

        expect(status).toEqual("Succeeded");
    }, TEST_TIMEOUT);

    it('should pass if the version already exists', async () => {
        await api.create('test-4', { resource: 'res-match', pipeline: 'pipe-match', version: 'ver-match' });
        const test = await sidecar.runTest('test-4', 'ver-match', 'res-match', 'pipe-match');
        const status = await test.exitStatus();

        expect(status).toEqual("Succeeded");
    }, TEST_TIMEOUT);
});