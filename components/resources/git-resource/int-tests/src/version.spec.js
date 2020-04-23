const runner = require('./helpers/test-runner');
const FileInspector = require('./helpers/file-inspector');
const gitHelper = require('./helpers/git-helper');

const START_TIMEOUT = 30000;
const TEST_TIMEOUT = 20000;

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

    describe('first commit', () => {
        it('should create a version.txt file', async () => {
            await gitHelper.addCommitMessage('test-file-version-1.txt', 'a message', 'contents');

            const testName = 'version-test-1'
            await runner.runTest({ name: testName, action: 'version' });

            this.fileInspector = new FileInspector(testName);
            await this.fileInspector.waitUntilReady();

            const result = await this.fileInspector.list();
            expect(result.files).toEqual([ 'output/version.txt' ]);
        }, TEST_TIMEOUT);

        it('should get the latest version', async () => {
            const commit = await gitHelper.addCommitMessage('test-file-version-2.txt', 'another message', 'more contents');

            const testName = 'version-test-2'
            await runner.runTest({ name: testName, action: 'version' });

            this.fileInspector = new FileInspector(testName);
            await this.fileInspector.waitUntilReady();

            const result = await this.fileInspector.get('output/version.txt');

            expect(result.trim()).toEqual(commit);
        }, TEST_TIMEOUT);

        it('should not ignore files outside of FILTER_PATH', async () => {
            const commit = await gitHelper.addCommitMessage('test-file-version-3.txt', 'another message', 'more contents');

            const testName = 'version-test-3'
            await runner.runTest({ name: testName, action: 'version', envExtras: [ { name: 'FILTER_PATH', value: 'subdir' } ] });

            this.fileInspector = new FileInspector(testName);
            await this.fileInspector.waitUntilReady();

            const result = await this.fileInspector.get('output/version.txt');
            expect(result.trim()).toEqual(commit);
        }, TEST_TIMEOUT);
    });

    describe('second commit', () => {
        it('should create a version.txt file', async () => {
            const previousCommit = await gitHelper.addCommitMessage('test-file-version-4.txt', 'a message', 'contents');
            await gitHelper.addCommitMessage('test-file-version-5.txt', 'a message', 'contents');

            const testName = 'version-test-4'
            await runner.runTest({ name: testName, action: 'version', envExtras: [ { name: 'PREVIOUS_VERSION', value: previousCommit } ] });

            this.fileInspector = new FileInspector(testName);
            await this.fileInspector.waitUntilReady();

            const result = await this.fileInspector.list();
            expect(result.files).toEqual([ 'output/version.txt' ]);
        }, TEST_TIMEOUT);

        it('should get the updated version', async () => {
            const previousCommit = await gitHelper.addCommitMessage('test-file-version-6.txt', 'a message', 'contents');
            const currentCommit = await gitHelper.addCommitMessage('test-file-version-7.txt', 'a message', 'contents');

            const testName = 'version-test-5'
            await runner.runTest({ name: testName, action: 'version', envExtras: [ { name: 'PREVIOUS_VERSION', value: previousCommit } ] });

            this.fileInspector = new FileInspector(testName);
            await this.fileInspector.waitUntilReady();

            const result = await this.fileInspector.get('output/version.txt');
            expect(result.trim()).toEqual(currentCommit);
        }, TEST_TIMEOUT);
    });
});