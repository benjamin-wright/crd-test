const git = require('simple-git/promise');

const user = process.env['TEST_USER'];
const repo = process.env['TEST_REPO'];

const connectionString = `ssh://${user}@git-repo/git/${repo}`;

describe('repo', () => {
    let gitInstance = null;

    beforeEach(() => {
        gitInstance = git();
    });

    it('should allow clone', async () => {
        await gitInstance.clone(connectionString, "./tmp/checkout");
    }, 20000);
});
