module.exports = {
    resource,
    pipeline,
    cronJob
}

function resource(name) {
    return {
        apiVersion: 'minion.ponglehub.com/v1',
        kind: 'Resource',
        metadata: {
            name
        },
        spec: {
            image: 'localhost/my-image'
        }
    };
}

function pipeline({ pipeline, resource, trigger }) {
    return {
        apiVersion: 'minion.ponglehub.com/v1',
        kind: 'Pipeline',
        metadata: {
            name: pipeline
        },
        spec: {
            resources: [
                {
                    name: resource,
                    trigger,
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
}

function cronJob(name, pipeline, resource) {
    return {
        apiVersion: "batch/v1beta1",
        kind: "CronJob",
        metadata: {
            name: name,
            labels: {
                pipeline,
                resource,
                "minion-type": "resource-watcher"
            },
            annotations: {
                "minion.ponglehub.co.uk/pipeline": pipeline,
                "minion.ponglehub.co.uk/resource": resource,
                "minion.ponglehub.co.uk/image": "localhost/some-image",
                "minion.ponglehub.co.uk/minion-type": "resource-watcher",
            }
        },
        spec: {
            schedule: "* * * * *",
            jobTemplate: {
                spec: {
                    template: {
                        metadata: {
                            labels: {
                                app: name
                            }
                        },
                        spec: {
                            containers: [
                                {
                                    name: name,
                                    image: "localhost/some-image",
                                    command: ["./version"]
                                }
                            ],
                            restartPolicy: "Never"
                        }
                    }
                }
            }
        }
    };
}