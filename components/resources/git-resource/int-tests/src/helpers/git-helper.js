const git = require('simple-git/promise');
const env = require('./environment');
const fs = require('fs').promises;
const ssh = require('./ssh-helper');
const faker = require('faker');

const { forEach } = require('@minion-ci/async-tools');

const connectionString = `ssh://${env.user}@${env.host}/git/${env.repo}`;
const repoDir = `tmp/checkout-${faker.random.alphaNumeric(8)}`;

let rootCommit = null;

async function dirMissing(path) {
    try {
        await fs.access(path);
        return false;
    } catch {
        return true;
    }
}

module.exports = {
    init,
    addCommitMessage,
    getLatestCommit,
    getFiles,
    rootCommit: () => rootCommit
}

async function init() {
    if (await dirMissing('/root/.ssh')) {
        await fs.mkdir('/root/.ssh');
        await fs.writeFile('/root/.ssh/id_rsa', Buffer.from(env.sshKey, 'base64').toString(), { mode: 0o600 });
        await fs.writeFile('/root/.ssh/id_rsa.pub', Buffer.from(env.sshPublicKey, 'base64').toString());
        await ssh.addToKnownHosts(env.host);
    }

    const gitInstance = git();
    await gitInstance.clone(connectionString, repoDir);

    if (await numCommits() === 0) {
        const response = await addCommitMessage('root.txt', 'root commit', 'first commits are weird');
        rootCommit = response.commit;
    }
}

async function numCommits() {
    try {
        const repo = git(repoDir);
        const logs = await repo.log();
        return logs.total
    } catch {
        return 0;
    }
}

async function addCommitMessage(file, message, contents, branch = 'master') {
    const segments = file.split('/');
    if (segments.length > 1) {
        const dir = segments.splice(0, segments.length - 1).join('/');
        try {
            await fs.mkdir(`${repoDir}/${dir}`, { recursive: true });
        } catch (err) {
        }
    }

    const repo = git(repoDir);

    if (branch !== 'master') {
        await repo.checkoutBranch(branch, 'master');
    }

    await fs.writeFile(`${repoDir}/${file}`, contents);
    await repo.add(file);

    await repo.addConfig('user.name', process.env['USER']);
    await repo.addConfig('user.email', `${process.env['USER']}@email.org`);

    const commitResult = await repo.commit(message);

    try {
        await repo.push('origin', branch);
    } catch {
        await repo.push(['-u', 'origin', branch]);
    }

    const commit = commitResult.commit;
    const fullCommit = await repo.revparse([ commit.replace("(root-commit) ", "") ]);

    return fullCommit.trim();
}

async function getLatestCommit(branch) {
    const repo = git(repoDir);
    await repo.pull('origin', branch);

    const logs = await repo.log([ `origin/${branch}` ]);

    return logs.latest;
}

async function getFiles() {
    const files = await listDirectory(`${repoDir}`, '.');

    return files;
}

async function listDirectory(directory, root) {
    const names = await fs.readdir(`${root}/${directory}`);
    const paths = names.filter(
        name => !name.startsWith('.')
    ).map(
        name => `${root}/${name}`
    );

    const dirs = [];
    const files = [];

    await forEach(paths, async path => {
        const stats = await fs.stat(`${root}/${directory}/${path}`);

        if (stats.isDirectory()) { dirs.push(path); }
        if (stats.isFile()) { files.push(path); }
    });

    return files;
}