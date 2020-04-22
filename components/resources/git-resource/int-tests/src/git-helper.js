const git = require('simple-git/promise');
const env = require('./environment');
const fs = require('fs').promises;
const ssh = require('./ssh-helper');
const faker = require('faker');

const connectionString = `ssh://${env.user}@${env.host}/git/${env.repo}`;
const repoDir = `tmp/checkout-${faker.random.alphaNumeric(8)}`;

async function dirMissing(path) {
    try {
        await fs.access(path);
        return false;
    } catch {
        return true;
    }
}

module.exports = {
    init: async () => {
        if (await dirMissing('/root/.ssh')) {
            await fs.mkdir('/root/.ssh');
            await fs.writeFile('/root/.ssh/id_rsa', Buffer.from(env.sshKey, 'base64').toString(), { mode: 0o600 });
            await fs.writeFile('/root/.ssh/id_rsa.pub', Buffer.from(env.sshPublicKey, 'base64').toString());
            await ssh.addToKnownHosts(env.host);
        }

        const gitInstance = git();
        await gitInstance.clone(connectionString, repoDir);
    },
    addCommitMessage: async (file, message, contents) => {
        const repo = git(repoDir);
        await fs.writeFile(`${repoDir}/${file}`, contents);
        await repo.add(file);

        await repo.addConfig('user.name', process.env['USER']);
        await repo.addConfig('user.email', `${process.env['USER']}@email.org`);

        const commitResult = await repo.commit(message);
        await repo.push('origin', 'master');

        return commitResult.commit;
    }
}