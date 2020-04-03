
const Client = require('kubernetes-client').Client;
const Request = require('kubernetes-client/backends/request');

class ApiHelper {
    constructor(namespace = 'default') {
        this.backend = new Request(Request.config.getInCluster());
        this.namespace = namespace;
    }

    async init() {
        this.client = new Client({ backend: this.backend });
        await this.client.loadSpec();
    }

    async getCronJobs() {
        const result = await this.client.apis.batch.v1beta1.namespaces(this.namespace).cronjobs.get();

        if (result.statusCode !== 200)

        throw new Error(`Failed to fetch cronJobs: ${result.statusCode}`);

        return result.body.items;
    }

    async addResource(name) {
        const resource = {
            apiVersion: 'minion.ponglehub.com/v1',
            kind: 'Resource',
            metadata: {
                name
            },
            spec: {
                image: 'localhost/my-image'
            }
        }

        return await this.client.apis['minion.ponglehub.com'].v1.namespaces(this.namespace).resources.post({ body: resource });
    }

    async addPipeline(name) {
        const pipeline = {
            apiVersion: 'minion.ponglehub.com/v1',
            kind: 'Pipeline',
            metadata: {
                name
            },
            spec: {
                resources: [
                    {
                        name: 'git-resource',
                        trigger: true,
                        secrets: [
                            {
                                name: 'my-confg',
                                keys: [
                                    { key: 'id-rsa.pub', path: '/root/.ssh' }
                                ]
                            }
                        ],
                        env: {
                            REPO: 'git@github.com:username/repo.git'
                        }
                    }
                ],
                steps: [
                    {
                        name: 'Load source',
                        resource: 'my-resource',
                        action: 'GET',
                        path: 'some/sub/path'
                    }
                ]
            }
        };

        return await this.client.apis['minion.ponglehub.com'].v1.namespaces(this.namespace).pipelines.post({ body: pipeline });
    }
}

module.exports = ApiHelper;