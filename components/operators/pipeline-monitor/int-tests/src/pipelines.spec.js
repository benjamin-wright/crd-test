const ApiHelper = require('./api-helper');

describe('I\'m a test!', () => {
    let apiHelper;

    beforeAll(() => {
        apiHelper = new ApiHelper();
    });

    it('should work!', async () => {
        await apiHelper.addPipeline('pipeline', 'my-pipelines');
    });
});