const faker = require('faker');
const ApiHelper = require('./api-helper');
const wait = require('./wait-helper');
const namespace = process.env["TEST_NAMESPACE"];
const TIMEOUT = 60000;
const POLLING_TIMEOUT = 20000;

describe('I\'m a test!', () => {
    let apiHelper = new ApiHelper(namespace);
    let pipeline;
    let resource;
    let image;

    beforeAll(async () => {
        await apiHelper.init();
    });

    beforeEach(() => {
        pipeline = faker.lorem.word();
        resource = faker.lorem.word();
        image = `localhost/${faker.lorem.word()}:${faker.lorem.word()}`;
    });

    describe('pipeline does not exist', () => {
        it('should add a cronjob to monitor the resource for a pipeline', async () => {
            await apiHelper.addResource({ resource, image });
            await apiHelper.addPipeline({ pipeline, resource, trigger: true });

            await wait.forSuccess(async () => await apiHelper.getCronJob(`${pipeline}-${resource}`));

            const cronJob = await apiHelper.getCronJob(`${pipeline}-${resource}`);
            expect(cronJob.spec.jobTemplate.spec.template.spec.containers.map(c => c.image)).toEqual([ image ]);
        }, TIMEOUT);

        it('should not add a cronjob to monitor a non-triggering resource for a pipeline', async () => {
            await apiHelper.addResource({ resource, image });
            await apiHelper.addPipeline({ pipeline, resource, trigger: false });

            await wait.forSuccess(async () => {
                await expect(apiHelper.getCronJob(`${pipeline}-${resource}`)).rejects.toEqual(new Error(`cronjobs.batch "${pipeline}-${resource}" not found`));
            }, POLLING_TIMEOUT);
        }, TIMEOUT);
    });

    describe('pipeline exists', () => {
        beforeEach(async () => {
            await apiHelper.addCronJob(`${pipeline}-${resource}`, pipeline, resource, image);
            await apiHelper.addResource({ resource, image });
            await apiHelper.addPipeline({ pipeline, resource, trigger: true });

            await wait.forSuccess(async () => await apiHelper.getCronJob(`${pipeline}-${resource}`));
        }, TIMEOUT);

        it('should remove the resource watcher when the pipeline is deleted', async () => {
            await apiHelper.deletePipeline(pipeline);
            await wait.forSuccess(async () => {
                await expect(apiHelper.getCronJob(`${pipeline}-${resource}`)).rejects.toEqual(new Error(`cronjobs.batch "${pipeline}-${resource}" not found`));
            });
        }, TIMEOUT);
    });
});