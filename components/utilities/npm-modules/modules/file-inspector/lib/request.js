const http = require('http');

module.exports = {
    call
}

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