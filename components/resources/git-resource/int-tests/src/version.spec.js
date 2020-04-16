const runner = require('./test-runner');
const FileInspector = require('./file-inspector');
const RepoManager = require('./repo-manager');

describe('version', () => {
    beforeAll(async () => {
        await runner.init();
    });

    afterEach(async () => {
        await this.fileInspector.exit();
    });

    it('should create a version.txt file', async () => {
        const repoManager = new RepoManager();
        await repoManager.waitUntilReady();

        const testName = 'version-test-1'
        await runner.runTest({ name: testName, action: 'version' });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.list();
        expect(result.files).toEqual([ 'input/version.txt' ]);
    });

    it.skip('should get the latest version', async () => {
        const testName = 'version-test-2'
        await runner.runTest({ name: testName, action: 'version' });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.get('input/version.txt');
        expect(result).toMatch(/^[a-zA-Z0-9]{40}(\r\n|\r|\n)$/)
    });
});