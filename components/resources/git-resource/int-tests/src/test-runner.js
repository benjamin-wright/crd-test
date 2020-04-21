const gitResourceImage = process.env['GIT_RESOURCE_IMAGE'];
const fileInspectorImage = process.env['FILE_INSPECTOR_IMAGE'];
const env = require('./environment');

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
    await client.api.v1.namespaces(env.namespace).secrets.post({ body: getSecretBody(`${name}-ssh-keys`, env.sshKey, env.sshPublicKey) });
    await client.apis.batch.v1.namespaces(env.namespace).jobs.post({ body: getJobBody(name, action, `${name}-ssh-keys`) });
    await client.api.v1.namespaces(env.namespace).services.post({ body: getServiceBody(name) });
}

function getJobBody(name, action, secret) {
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
            backoffLimit: 0,
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
                                { name: 'REPO', value: `ssh://${env.user}@${env.host}/git/${env.repo}` },
                                { name: 'REPO_HOST', value: env.host },
                                { name: 'BRANCH', value: env.branch },
                            ],
                            volumeMounts: [
                                {
                                    name: 'inputs',
                                    mountPath: '/input'
                                },
                                {
                                    name: 'outputs',
                                    mountPath: '/output'
                                },
                                {
                                    name: 'ssh',
                                    mountPath: '/data/ssh',
                                    readOnly: true
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
                        },
                        {
                            name: 'ssh',
                            secret: {
                                secretName: secret
                            }
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

function getSecretBody(name, sslPrivateKey, sslPublicKey) {
    return {
        apiVersion: 'v1',
        kind: 'Secret',
        metadata: {
            name
        },
        data: {
            'id_rsa': sslPrivateKey,
            'id_rsa.pub': sslPublicKey
        }
    }
}