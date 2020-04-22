const runner = require('./helpers/test-runner');
const FileInspector = require('./helpers/file-inspector');
const gitHelper = require('./helpers/git-helper');

const START_TIMEOUT = 20000;
const TEST_TIMEOUT = 15000;

describe('load', () => {
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

    it('should clone into the input dir', async () => {
        await gitHelper.addCommitMessage('test-file-load-1.txt', 'another message', 'more contents');

        const testName = 'load-test-1'
        await runner.runTest({ name: testName, action: 'load' });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.list();
        expect(result.files).toContain('input/test-file-load-1.txt');
    }, TEST_TIMEOUT);

    it('should clone into a subdirectory if "CHECKOUT_DIR" is present', async () => {
        await gitHelper.addCommitMessage('test-file-load-2.txt', 'and another message', 'yet more contents');

        const testName = 'load-test-2'
        await runner.runTest({ name: testName, action: 'load', envExtras: [ { name: 'CHECKOUT_DIR', value: 'subdir' } ] });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const result = await this.fileInspector.list();
        expect(result.files).toContain('input/subdir/test-file-load-2.txt');
    }, TEST_TIMEOUT);
});