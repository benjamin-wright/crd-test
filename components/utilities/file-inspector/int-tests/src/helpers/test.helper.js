const axios = require('axios');
const http = require('http');

class Client {
    constructor() {
        this.host = process.env['TARGET_HOST'];
        this.port = process.env['TARGET_PORT'];
    }

    get(path) {
        return new Promise((resolve, reject) => {
            const options = {
                hostname: this.host,
                port: this.port,
                path: `/${path}`,
                method: 'GET'
            };

            const req = http.request(options, res => {
                let data = '';

                if (res.statusCode < 200 || res.statusCode > 299) {
                    return reject(new Error(`Error making request: ${res.statusCode}`));
                }

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
}

const client = new Client();

module.exports = {
    waitForSpinup,
    listContents,
    getFile
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
    await retry(async () => client.get('status'), {});
}

async function listContents() {
    try {
        const response = await client.get('list');

        return { status: response.status, data: response.data };
    } catch (err) {
        console.error(`Failed to list contents: ${err.message}`);
        throw err;
    }
}

async function getFile(filename) {
    try {
        const response = await client.get(`file/${filename}`);

        return {status: response.status, data: response.data };
    } catch (err) {
        console.error(`Failed to get file: ${err.message}`);
        throw err;
    }
}