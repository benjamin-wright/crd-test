const DEFAULT_TIMEOUT = 3000;
const DEFAULT_POLLING_PERIOD = 250;

module.exports = {
    forSuccess
}

function wait(timeout = DEFAULT_POLLING_PERIOD) {
    return new Promise(resolve => setTimeout(resolve, timeout));
}

async function forSuccess(test, timeout = DEFAULT_TIMEOUT) {
    let running = true;
    let lastError = null;
    const timer = setTimeout(() => running = false, timeout);

    while (running) {
        try {
            await test();
            clearTimeout(timer);
            return;
        } catch (err) {
            lastError = err;
            await wait();
        }
    }

    throw lastError;
}