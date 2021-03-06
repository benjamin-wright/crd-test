const runner = require('./helpers/test-runner');
const FileInspector = require('@minion-ci/file-inspector');
const gitHelper = require('./helpers/git-helper');

const START_TIMEOUT = 30000;
const TEST_TIMEOUT = 20000;

const env = require('./helpers/environment');

describe('load', () => {
    beforeAll(async () => {
        await runner.init();
        await gitHelper.init();
    }, START_TIMEOUT);

    afterEach(async () => {
        if (this.fileInspector) {
            await this.fileInspector.exit();
        }
    });

    it('should clone into the output dir', async () => {
        const commit = await gitHelper.addCommitMessage('test-file-load-1.txt', 'another message', 'more contents');

        const testName = 'load-test-1'
        await runner.runTest({ name: testName, action: 'load', envExtras: [ { name: 'CURRENT_VERSION', value: commit } ] });

        this.fileInspector = new FileInspector(testName, env.namespace);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.list();
        expect(result.files).toContain('output/test-file-load-1.txt');
    }, TEST_TIMEOUT);

    it('should clone into a subdirectory if "CHECKOUT_DIR" is present', async () => {
        const commit = await gitHelper.addCommitMessage('test-file-load-2.txt', 'and another message', 'yet more contents');

        const testName = 'load-test-2'
        await runner.runTest({ name: testName, action: 'load', envExtras: [ { name: 'CURRENT_VERSION', value: commit }, { name: 'CHECKOUT_DIR', value: 'subdir' } ] });

        this.fileInspector = new FileInspector(testName, env.namespace);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.list();
        expect(result.files).toContain('output/subdir/test-file-load-2.txt');
    }, TEST_TIMEOUT);

    it('should clone an older commit if "CURRENT_VERSION" is set', async () => {
        const commit = await gitHelper.addCommitMessage('test-file-load-3.txt', 'and another message', 'yet more contents');
        await gitHelper.addCommitMessage('test-file-load-4.txt', 'and another message', 'yet more contents');

        const testName = 'load-test-3'
        await runner.runTest({ name: testName, action: 'load', envExtras: [ { name: 'CURRENT_VERSION', value: commit } ] });

        this.fileInspector = new FileInspector(testName, env.namespace);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.list();
        expect(result.files).toContain('output/test-file-load-3.txt');
        expect(result.files).not.toContain('output/test-file-load-4.txt');
    }, TEST_TIMEOUT);
});