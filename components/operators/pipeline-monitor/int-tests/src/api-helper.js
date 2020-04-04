
const Client = require('kubernetes-client').Client;
const Request = require('kubernetes-client/backends/request');
const templates = require('./templates');

class ApiHelper {
    constructor(namespace = 'default') {
        this.backend = new Request(Request.config.getInCluster());
        this.namespace = namespace;
    }

    async init() {
        this.client = new Client({ backend: this.backend });
        await this.client.loadSpec();
    }

    async getCronJob(name) {
        const result = await this.client.apis.batch.v1beta1.namespaces(this.namespace).cronjobs(name).get()

        if (result.statusCode !== 200)

        throw new Error(`Failed to fetch cronJob: ${result.statusCode}`);

        return result.body;
    }

    async getCronJobs() {
        const result = await this.client.apis.batch.v1beta1.namespaces(this.namespace).cronjobs.get();

        if (result.statusCode !== 200)

        throw new Error(`Failed to fetch cronJobs: ${result.statusCode}`);

        return result.body.items;
    }

    async addResource(name) {
        const resource = templates.resource(name);

        return await this.client.apis['minion.ponglehub.com'].v1.namespaces(this.namespace).resources.post({ body: resource });
    }

    async addPipeline(name, resource) {
        const pipeline = templates.pipeline(name, resource);

        return await this.client.apis['minion.ponglehub.com'].v1.namespaces(this.namespace).pipelines.post({ body: pipeline });
    }

    async deletePipeline(name) {
        return await this.client.apis['minion.ponglehub.com'].v1.namespaces(this.namespace).pipelines(name).delete();
    }

    async addCronJob(name, pipeline, resource) {
        const cronjob = templates.cronJob(name, pipeline, resource);

        return await this.client.apis.batch.v1beta1.namespaces(this.namespace).cronjobs.post({ body: cronjob });
    }
}

module.exports = ApiHelper;