const ApiHelper = require('./api-helper');
const wait = require('./wait-helper');
const namespace = process.env["TEST_NAMESPACE"];
const TIMEOUT = 60000;

describe('I\'m a test!', () => {
    let apiHelper = new ApiHelper(namespace);

    beforeAll(async () => {
        await apiHelper.init();
    });

    it('should add a cronjob to monitor the pipeline', async () => {
        await apiHelper.addResource('git-resource-1');
        await apiHelper.addPipeline('pipeline-1', 'git-resource-1');

        await wait.forSuccess(async () => await apiHelper.getCronJob('pipeline-1-git-resource-1'));
    }, TIMEOUT);

    it('should remove a cronjob when the pipeline is deleted', async () => {
        const resource = 'git-resource-2';
        const pipeline = 'pipeline-2';

        await apiHelper.addCronJob(`${pipeline}-${resource}`, pipeline, resource);
        await apiHelper.addResource(resource);
        await apiHelper.addPipeline(pipeline, resource);

        await wait.forSuccess(async () => await apiHelper.getCronJob(`${pipeline}-${resource}`));
        await apiHelper.deletePipeline(pipeline);
        await wait.forSuccess(async () => {
            await expect(apiHelper.getCronJob(`${pipeline}-${resource}`)).rejects.toEqual({});
        });
    }, TIMEOUT);
});