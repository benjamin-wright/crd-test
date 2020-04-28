const env = require('./environment');
const manifests = require('./manifests');

const { sleep } = require('@minion-ci/async-tools');

const { Client, KubeConfig } = require('kubernetes-client');
const Request = require('kubernetes-client/backends/request');
const kubeconfig = new KubeConfig();
kubeconfig.loadFromCluster();
const backend = new Request({ kubeconfig });
const client = new Client({ backend });

module.exports = {
    init,
    runTest
}

async function init() {
    await client.loadSpec();
}

async function runTest(name, version) {
    const manifest = manifests.test(env.testNamespace, name, env.sidecarImage, version);
    const result = await client.apis.batch.v1.namespaces(env.testNamespace).jobs.post({ body: manifest });

    console.log(JSON.stringify(result, null, 2));

    return new Sidecar(name);
}

class Sidecar {
    constructor(name) {
        this.name = name;
    }

    async exitStatus() {
        while (true) {
            const job = await client.apis.batch.v1.namespaces(env.testNamespace).jobs(this.name).get();

            if (job.statusCode !== 200) {
                throw new Error(`Failed to fetch test job with status code: ${job.statusCode}`);
            }

            if (job.body.status.failed > 0) {
                return "Failed"
            }

            if (job.body.status.succeeded > 0) {
                return "Succeeded"
            }

            await sleep(50);
        }
    }
}
