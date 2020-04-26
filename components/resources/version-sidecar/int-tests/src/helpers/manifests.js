module.exports = {
    test
}

function test(name, image, version) {
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
                    initContainers: (version !== undefined ? [ versionInjector ] : []),
                    containers: [
                        {
                            name: 'sidecar',
                            image: image,
                            env: [
                                { name: 'NAME', value: 'VALUE' }
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