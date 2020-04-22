const { spawn } = require('child_process');

function runProcess(command) {
    return new Promise((resolve, reject) => {
        const proc = spawn('/bin/sh', [ '-c', command ]);

        let stdout = '';
        let stderr = '';

        proc.stdout.on('data', data => {
            stdout += `${data}`;
        });

        proc.stderr.on('data', data => {
            stderr += `${data}`;
        });

        proc.on('close', code => {
            if (code == 0) {
                return resolve();
            } else {
                console.error(stderr);
                return reject(code);
            }
        });
    });
}

module.exports = {
    addToKnownHosts: async (host) => {
        await runProcess(`ssh-keyscan -H ${host} >> /root/.ssh/known_hosts`);
    }
}