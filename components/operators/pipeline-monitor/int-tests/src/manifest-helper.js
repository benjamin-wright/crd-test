module.exports = {
    getCronContainers
}

function getCronContainers(cronJob) {
    const spec = cronJob.spec.jobTemplate.spec.template.spec;
    return [
        ...(spec.containers ? spec.containers : []),
        ...(spec.initContainers ? spec.initContainers : []),
    ];
}