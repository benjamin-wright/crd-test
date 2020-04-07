module.exports = {
    resource,
    pipeline,
    cronJob
}

function resource({ resource, image }) {
    return {
        apiVersion: 'minion.ponglehub.com/v1',
        kind: 'Resource',
        metadata: {
            name: resource
        },
        spec: {
            image,
            secrets: [
                {
                    name: 'my-config',
                    mountPath: '/root/.ssh',
                    keys: [
                        { key: 'id-rsa.pub', path: 'id-rsa.pub' }
                    ]
                }
            ],
            env: [
                { name: 'REPO', value: 'git@github.com:username/repo.git' }
            ]
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
                    trigger
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

function cronJob(name, pipeline, resource, image) {
    return {
        apiVersion: "batch/v1beta1",
        kind: "CronJob",
        metadata: {
            name,
            labels: {
                pipeline,
                resource,
                "minion-type": "resource-watcher"
            },
            annotations: {
                "minion.ponglehub.co.uk/pipeline": pipeline,
                "minion.ponglehub.co.uk/resource": resource,
                "minion.ponglehub.co.uk/image": image,
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
                                    name,
                                    image,
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