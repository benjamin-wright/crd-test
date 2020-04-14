const axios = require('axios');

class Client {
    constructor() {
        this.client = axios.create({
            baseURL: `${process.env['TARGET_URL']}:${process.env['TARGET_PORT']}`,
            timeout: 1000
        });
    }

    async get(url) {
        const errFunc = console.error;
        console.error = () => {};
        let attempts = 0;

        while (true) {
            try {
                const response = await axios.get(`${process.env['TARGET_URL']}:${process.env['TARGET_PORT']}/${url}`);
                console.error = errFunc;
                return response;
            } catch (err) {
                attempts += 1;

                if (attempts > 5) {
                    console.error = errFunc;
                    throw err;
                }

                await sleep(10);
            }
        }
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
    await sleep(1000);
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