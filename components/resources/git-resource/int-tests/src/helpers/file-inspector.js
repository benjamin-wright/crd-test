const http = require('http');
const namespace = require('./environment').namespace;

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

module.exports = class FileInspector {
    constructor(name) {
        this.url = `http://${name}.${namespace}.svc.cluster.local`;
    }

    async waitUntilReady() {
        while (true) {
            try {
                const response = await call(`${this.url}/status`, 'GET');
                if (response.status === 200) {
                    return;
                };
            } catch (err) {
            }

            await sleep(200);
        }
    }

    async list() {
        const response = await call(`${this.url}/list`, 'GET');

        if (response.status !== 200) {
            throw new Error(`Error getting files list: ${response.status}`);
        }

        return response.data;
    }

    async get(file) {
        const response = await call(`${this.url}/file/${file}`, 'GET');

        if (response.status !== 200) {
            throw new Error(`Error getting file "${file}": ${response.status}`);
        }

        return response.data;
    }

    async exit() {
        const response = await call(`${this.url}/exit`, 'POST');

        if (response.status !== 202) {
            throw new Error(`Error shutting down the file-inspector: ${response.status}`);
        }

        return response.data;
    }
}