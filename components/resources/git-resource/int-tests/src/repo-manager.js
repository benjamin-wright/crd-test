const http = require('http');
const REPO = process.env['PARENT_REPO'];

async function call(url, method) {
    return new Promise((resolve, reject) => {
        const options = {
            method
        };

        const req = http.request(url, options, res => {
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

async function sleep(timeout) {
    return new Promise(resolve => setTimeout(resolve, timeout));
}

module.exports = class RepoManager {
    constructor() {
        this.url = `${REPO}/api/v1`;
    }

    async waitUntilReady() {
        const response = await call(`${this.url}/user/repos`, 'GET');
        console.log(response);

        throw new Error('Hi!');
    }
}