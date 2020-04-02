
const Client = require('kubernetes-client').Client;
const Request = require('kubernetes-client/backends/request');

class ApiHelper {
    constructor() {
        const backend = new Request(Request.config.getInCluster());
        this.client = new Client({ backend }).loadSpec();
    }

    async addPipeline(name, namespace) {
        const pipeline = JSON.stringify({
            apiVersion: "v1",
            kind: "Pipeline",
            metadata: {
                name,
                namespace
            },
            spec: {
                resources: [
                    {
                        name: "git-resource",
                        trigger: true,
                        secrets: [
                            {
                                name: "my-confg",
                                keys: [
                                    { key: "id-rsa.pub", path: "/root/.ssh" }
                                ]
                            }
                        ],
                        env: {
                            REPO: "git@github.com:username/repo.git"
                        }
                    }
                ],
                steps: [
                    {
                        name: "Load source",
                        resource: "my-resource",
                        action: "GET",
                        path: "some/sub/path"
                    }
                ]
            }
        });

        return await this.client.apis['minion.ponglehub.com'].v1.namespaces(namespace).pipelines.post({ body: pipeline });
    }
}

module.exports = ApiHelper;