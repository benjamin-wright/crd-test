const ApiHelper = require('./api-helper');
const wait = require('./wait-helper');
const namespace = process.env["TEST_NAMESPACE"];
const TIMEOUT = 60000;
const POLLING_TIMEOUT = 20000;

describe('I\'m a test!', () => {
    let apiHelper = new ApiHelper(namespace);

    beforeAll(async () => {
        await apiHelper.init();
    });

    describe('pipeline does not exist', () => {
        it('should add a cronjob to monitor the resource for a pipeline', async () => {
            const resource = 'git-resource-1';
            const pipeline = 'pipeline-1';

            await apiHelper.addResource(resource);
            await apiHelper.addPipeline({ pipeline, resource, trigger: true });

            await wait.forSuccess(async () => await apiHelper.getCronJob('pipeline-1-git-resource-1'));
        }, TIMEOUT);

        it('should not add a cronjob to monitor a non-triggering resource for a pipeline', async () => {
            const resource = 'git-resource-3';
            const pipeline = 'pipeline-3';

            await apiHelper.addResource(resource);
            await apiHelper.addPipeline({ pipeline, resource, trigger: false });

            await wait.forSuccess(async () => {
                await expect(apiHelper.getCronJob(`${pipeline}-${resource}`)).rejects.toEqual(new Error('cronjobs.batch "pipeline-3-git-resource-3" not found'));
            }, POLLING_TIMEOUT);
        }, TIMEOUT);
    });

    describe('pipeline exists', () => {
        const resource = 'git-resource-2';
        const pipeline = 'pipeline-2';

        beforeEach(async () => {
            await apiHelper.addCronJob(`${pipeline}-${resource}`, pipeline, resource);
            await apiHelper.addResource(resource);
            await apiHelper.addPipeline({ pipeline, resource, trigger: true });

            await wait.forSuccess(async () => await apiHelper.getCronJob(`${pipeline}-${resource}`));
        }, TIMEOUT);

        it('should remove the resource watcher when the pipeline is deleted', async () => {
            await apiHelper.deletePipeline(pipeline);
            await wait.forSuccess(async () => {
                await expect(apiHelper.getCronJob(`${pipeline}-${resource}`)).rejects.toEqual(new Error('cronjobs.batch "pipeline-2-git-resource-2" not found'));
            });
        }, TIMEOUT);
    });
});