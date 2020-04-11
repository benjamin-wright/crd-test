const axios = require('axios');

const client = axios.create({
    baseURL: `${process.env['TARGET_URL']}:${process.env['TARGET_PORT']}`,
    timeout: 1000
});

module.exports = {
    waitForSpinup
}

async function sleep(timeout) {
    return new Promise(resolve => setTimeout(resolve, timeout));
}

async function retry(promise, { timeout = 10000, poll = 250 }) {
    const startMillis = Date.now();

    while(true) {
        try {
            await promise();
            break;
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
    const errFunc = console.error;
    console.error = () => {};

    const response = await retry(async () => client.get('/status'), {});

    console.error = errFunc;
    return response;
}