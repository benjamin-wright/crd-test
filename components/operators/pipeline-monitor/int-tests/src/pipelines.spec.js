const ApiHelper = require('./api-helper');
const wait = require('./wait-helper');
const namespace = process.env["TEST_NAMESPACE"];

describe('I\'m a test!', () => {
    let apiHelper = new ApiHelper(namespace);

    beforeAll(async () => {
        await apiHelper.init();
    });

    it('should work!', async () => {
        await apiHelper.addPipeline('pipeline');

        await wait.forSuccess(async () => {
            const jobs = await apiHelper.getCronJobs();
            expect(jobs.length).toBe(1);
        });
    });
});