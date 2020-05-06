const env = require('./environment');
const manifests = require('./manifests');

const { Client, KubeConfig } = require('kubernetes-client');
const Request = require('kubernetes-client/backends/request');
const kubeconfig = new KubeConfig();
kubeconfig.loadFromCluster();
const backend = new Request({ kubeconfig });
const client = new Client({ backend });

module.exports = {
    init,
    create
}

async function init() {
    await client.loadSpec();
}

async function create(name, { resource, pipeline, version }) {
    const manifest = manifests.version(name, resource, pipeline, version);
    const result = await client.apis['minion.ponglehub.com'].v1.namespaces(env.testNamespace).versions.post({ body: manifest });

    if (result.statusCode !== 201) {
        throw new Error(`Failed to post version with status code: ${result.statusCode}`);
    }
}