module.exports = {
    sleep,
    forEach
}

async function sleep(timeout) {
    return new Promise(resolve => setTimeout(resolve, timeout));
}

async function forEach(array, callback) {
    for (let i = 0; i < array.length(); i++) {
        await callback(array[i]);
    }
}