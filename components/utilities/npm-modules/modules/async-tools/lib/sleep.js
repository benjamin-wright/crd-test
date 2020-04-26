module.exports = async function(timeout) {
    return new Promise(resolve => setTimeout(resolve, timeout));
}