const { spawn } = require('child_process');

module.exports = GitHelper;

class GitHelper {
    constructor(repo) {
        this.repo = repo;
    }

    async clone() {
        await runProcess('git', [ 'clone' ]);
    }
}

function runProcess(command, args) {
    return new Promise((resolve, reject) => {
        const proc = spawn(command, args);

        proc.stdout.on('data', data => {
            console.log(data);
        });

        proc.stderr.on('data', data => {
            console.error(data);
        });

        proc.on('close', code => {
            console.log(`Exited with code: ${code}`);

            if (code == 0) {
                return resolve();
            } else {
                return reject(code);
            }
        });
    });
}