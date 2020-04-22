const runner = require('./helpers/test-runner');
const FileInspector = require('./helpers/file-inspector');
const gitHelper = require('./helpers/git-helper');

const START_TIMEOUT = 20000;
const TEST_TIMEOUT = 10000;

describe('version', () => {
    beforeAll(async () => {
        try {
            await runner.init();
            await gitHelper.init();
        } catch (err) {
            console.error(`Init error: ${err}`);
        }
    }, START_TIMEOUT);

    afterEach(async () => {
        if (this.fileInspector) {
            await this.fileInspector.exit();
        }
    });

    it('should create a version.txt file', async () => {
        await gitHelper.addCommitMessage('test-file-version-1.txt', 'a message', 'contents');

        const testName = 'version-test-1'
        await runner.runTest({ name: testName, action: 'version' });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.list();
        expect(result.files).toEqual([ 'input/version.txt' ]);
    }, TEST_TIMEOUT);

    it('should get the latest version', async () => {
        const commit = await gitHelper.addCommitMessage('test-file-version-2.txt', 'another message', 'more contents');

        const testName = 'version-test-2'
        await runner.runTest({ name: testName, action: 'version' });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.get('input/version.txt');
        expect(result.substring(0, 7)).toEqual(commit);
    }, TEST_TIMEOUT);
});