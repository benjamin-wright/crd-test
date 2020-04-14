const axios = require('axios');
const http = require('http');

class Client {
    constructor() {
        this.host = process.env['TARGET_HOST'];
        this.port = process.env['TARGET_PORT'];
    }

    call(path, method) {
        return new Promise((resolve, reject) => {
            const options = {
                hostname: this.host,
                port: this.port,
                path: `/${path}`,
                method
            };

            const req = http.request(options, res => {
                let data = '';

                res.on('data', chunk => {
                    data += chunk;
                });

                res.on('end', () => {
                    let body = null;
                    switch (res.headers['content-type']) {
                        case 'application/json':
                            body = JSON.parse(data);
                            break;
                        default:
                            body = data;
                            break;
                    }

                    return resolve({
                        status: res.statusCode,
                        data: body
                    });
                });
            });

            req.on('error', error => {
                reject(error);
            });

            req.end();
        });
    }

    get(path) {
        return this.call(path, 'GET');
    }

    post(path) {
        return this.call(path, 'POST');
    }
}

const client = new Client();

module.exports = {
    waitForSpinup,
    waitForRestart,
    listContents,
    getFile,
    getUptime,
    callExit
}

async function sleep(timeout) {
    return new Promise(resolve => setTimeout(resolve, timeout));
}

async function retry(promise, { timeout = 10000, poll = 250 }) {
    const startMillis = Date.now();

    while(true) {
        try {
            return await promise();
        } catch(err) {
            const elapsed = Date.now() - startMillis;
            if (elapsed > timeout) {
                throw new Error(`Server failed to start within ${Math.round(timeout / 1000)} seconds: ${err.message}`);
            }

            await sleep(poll);
        }
    }
}

async function waitForSpinup() {
    await retry(
        async () => {
            const response = await client.get('status');

            if (response.status !== 200) {
                throw new Error(`Expected 200 response: recieved ${response.status}`);
            }

            return response;
        }
        ,{}
    );
}

async function waitForRestart(lastUptime, timeout = 10000) {
    const startMillis = Date.now();

    try {
        while (true) {
            if (Date.now() > startMillis + timeout) {
                const err = new Error(`Server did not restart within ${Math.round(timeout / 1000)} seconds`);
                err.timeout = true;
                throw err;
            }

            await client.get('status');
            await sleep(500);
        }
    } catch (err) {
        if (err.timeout) {
            throw err;
        }
    }

    await waitForSpinup();
}

async function getUptime() {
    const response = await client.get('status');

    if (response.status !== 200) {
        throw new Error(`Failed to get status: ${response.status}`);
    }

    return response.data.Uptime;
}

async function listContents() {
    return await client.get('list');
}

async function getFile(filename) {
    return await client.get(`file/${filename}`);
}

async function callExit() {
    return await client.post('exit');
}