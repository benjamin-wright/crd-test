const runner = require('./helpers/test-runner');
const FileInspector = require('./helpers/file-inspector');
const gitHelper = require('./helpers/git-helper');

const START_TIMEOUT = 30000;
const TEST_TIMEOUT = 20000;

describe('update', () => {
    beforeAll(async () => {
        await runner.init();
        await gitHelper.init();
    }, START_TIMEOUT);

    afterEach(async () => {
        if (this.fileInspector) {
            await this.fileInspector.exit();
        }
    });

    it('should push the input file to the repository', async () => {
        const testName = 'update-test-1'
        await runner.runTest({ name: testName, action: 'update', input: { content: 'content', path: 'file1.txt' } });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const commit = await gitHelper.getLatestCommit('master');
        expect(commit.message).toEqual('updated by git-resource');
    }, TEST_TIMEOUT);

    it('should push to a new branch if that branch doesn\'t exist', async () => {
        const testName = 'update-test-2'
        await runner.runTest({ name: testName, action: 'update', input: { content: 'content', path: 'file2.txt' }, envExtras: [ { name: 'BRANCH', value: 'branch-1' } ] });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const commit = await gitHelper.getLatestCommit('branch-1');
        expect(commit.message).toEqual('updated by git-resource');
    }, TEST_TIMEOUT);

    it('should push to a new branch if that branch does exist', async () => {
        await gitHelper.addCommitMessage('test-file-update-3.txt', 'and another message', 'yet more contents', 'branch-2');

        const testName = 'update-test-3'
        await runner.runTest({ name: testName, action: 'update', input: { content: 'content', path: 'file3.txt' }, envExtras: [ { name: 'BRANCH', value: 'branch-2' } ] });

        this.fileInspector = new FileInspector(testName);
        await this.fileInspector.waitUntilReady();

        const commit = await gitHelper.getLatestCommit('branch-2');
        expect(commit.message).toEqual('updated by git-resource');
    }, TEST_TIMEOUT);
});