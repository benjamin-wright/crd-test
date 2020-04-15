const runner = require('./test-runner');
const FileInspector = require('./file-inspector');

describe('version', () => {
    beforeAll(async () => {
        await runner.init();
    });

    it('should create a version.txt file', async () => {
        const testName = 'version-test-1'
        await runner.runTest({ name: testName, action: 'version' });

        const fileInspector = new FileInspector(testName);
        await fileInspector.waitUntilReady();

        const result = await fileInspector.list();
        expect(result.files).toEqual([ 'input/version.txt' ]);
    });

    it('should get the latest version', async () => {
        const testName = 'version-test-2'
        await runner.runTest({ name: testName, action: 'version' });

        const fileInspector = new FileInspector(testName);
        await fileInspector.waitUntilReady();

        const result = await fileInspector.get('input/version.txt');
        expect(result).toEqual('v1');
    });
});