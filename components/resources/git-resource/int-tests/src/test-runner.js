const gitResourceImage = process.env['GIT_RESOURCE_IMAGE'];
const fileInspectorImage = process.env['FILE_INSPECTOR_IMAGE'];
const namespace = process.env['TEST_NAMESPACE'];
const testRepo = process.env['TEST_REPO'];
const testBranch = process.env['TEST_BRANCH'];

const Client = require('kubernetes-client').Client;
const Request = require('kubernetes-client/backends/request');

const backend = new Request(Request.config.getInCluster());
const client = new Client({ backend });

module.exports = {
    init,
    runTest
}

async function init() {
    await client.loadSpec();
}

async function runTest({ name, action }) {
    await client.apis.batch.v1.namespaces(namespace).jobs.post({ body: getJobBody(name, action) });
    await client.api.v1.namespaces(namespace).services.post({ body: getServiceBody(name) });
}

function getJobBody(name, action) {
    return {
        apiVersion: 'batch/v1',
        kind: 'Job',
        metadata: {
            name,
            labels: {
                app: name
            }
        },
        spec: {
            template: {
                metadata: {
                    labels: {
                        test: name
                    }
                },
                spec: {
                    initContainers: [
                        {
                            name: 'test',
                            image: gitResourceImage,
                            command: [ `./${action}` ],
                            env: [
                                { name: 'REPO', value: testRepo },
                                { name: 'BRANCH', value: testBranch },
                            ],
                            volumeMounts: [
                                {
                                    name: 'inputs',
                                    mountPath: '/input'
                                },
                                {
                                    name: 'outputs',
                                    mountPath: '/output'
                                }
                            ]
                        }
                    ],
                    containers: [
                        {
                            name: 'reporter',
                            image: fileInspectorImage,
                            volumeMounts: [
                                {
                                    name: 'inputs',
                                    mountPath: '/data/input'
                                },
                                {
                                    name: 'outputs',
                                    mountPath: '/data/output'
                                }
                            ]
                        }
                    ],
                    volumes: [
                        {
                            name: 'inputs',
                            emptyDir: {}
                        },
                        {
                            name: 'outputs',
                            emptyDir: {}
                        }
                    ],
                    restartPolicy: 'Never'
                }
            }
        }
    };
}

function getServiceBody(name) {
    return {
        kind: 'Service',
        apiVersion: 'v1',
        metadata: {
            name,
            labels: {
                test: name
            }
        },
        spec: {
            type: 'ClusterIP',
            selector: {
                test: name
            },
            ports: [
                {
                    port: 80,
                    targetPort: 80
                }
            ]
        }
    }
}