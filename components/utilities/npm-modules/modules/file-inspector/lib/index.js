const { sleep } = require('@minion-ci/async-tools');
const request = require('./request');

module.exports = class FileInspector {
    constructor(name, namespace) {
        this.url = `http://${name}.${namespace}.svc.cluster.local`;
    }

    async waitUntilReady() {
        while (true) {
            try {
                const response = await request.call(`${this.url}/status`, 'GET');
                if (response.status === 200) {
                    return;
                };
            } catch (err) {
            }

            await sleep(200);
        }
    }

    async list() {
        const response = await request.call(`${this.url}/list`, 'GET');

        if (response.status !== 200) {
            throw new Error(`Error getting files list: ${response.status}`);
        }

        return response.data;
    }

    async get(file) {
        const response = await request.call(`${this.url}/file/${file}`, 'GET');

        if (response.status !== 200) {
            throw new Error(`Error getting file "${file}": ${response.status}`);
        }

        return response.data;
    }

    async exit() {
        const response = await request.call(`${this.url}/exit`, 'POST');

        if (response.status !== 202) {
            throw new Error(`Error shutting down the file-inspector: ${response.status}`);
        }

        return response.data;
    }
}