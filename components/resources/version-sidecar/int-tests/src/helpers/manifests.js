module.exports = {
    test,
    version
}

function test(name, image, { version, namespace, resource, pipeline }) {
    const versionInjector = {
        name: 'setup',
        image: 'busybox',
        command: ['/bin/sh', '-c', `echo "${version}" > /output/version.txt`],
        volumeMounts: [
            {
                name: 'outputs',
                mountPath: '/output'
            }
        ]
    }

    return {
        apiVersion: 'batch/v1',
        kind: 'Job',
        metadata: {
            name: name
        },
        spec: {
            backoffLimit: 0,
            template: {
                spec: {
                    serviceAccount: 'version-sidecar',
                    initContainers: (version !== undefined ? [ versionInjector ] : []),
                    containers: [
                        {
                            name: 'sidecar',
                            image: image,
                            env: [
                                { name: 'NAMESPACE', value: namespace },
                                { name: 'RESOURCE', value: resource },
                                { name: 'PIPELINE', value: pipeline }
                            ],
                            volumeMounts: [
                                {
                                    name: 'outputs',
                                    mountPath: '/output'
                                }
                            ]
                        }
                    ],
                    restartPolicy: 'Never',
                    volumes: [
                        {
                            name: 'outputs',
                            emptyDir: {}
                        }
                    ]
                }
            }
        }
    }
}

function version(name, resource, pipeline, version) {
    return {
        apiVersion: 'minion.ponglehub.com/v1',
        kind: 'Version',
        metadata: {
            name
        },
        spec: {
            resource,
            pipeline,
            version
        }
    }
}
