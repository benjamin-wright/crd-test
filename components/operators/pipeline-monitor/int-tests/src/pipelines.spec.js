const faker = require('faker');

const ApiHelper = require('./api-helper');
const manifestHelper = require('./manifest-helper');
const wait = require('./wait-helper');

const namespace = process.env["TEST_NAMESPACE"];
const TIMEOUT = 60000;

const words = [];
function randomWord() {
    while (true) {
        const word = faker.lorem.word();

        if (words.includes(word)) {
            continue;
        }

        words.push(word);
        return word;
    }
}

describe('Pipeline Monitor', () => {
    let apiHelper = new ApiHelper(namespace);
    let pipeline;
    let resource;
    let image;

    beforeAll(async () => {
        await apiHelper.init();
    });

    beforeEach(() => {
        pipeline = randomWord();
        resource = randomWord();
        image = `localhost/${randomWord()}:${randomWord()}`;
    });

    describe('pipeline does not exist', () => {
        it('should add a cronjob to monitor the resource for a pipeline', async () => {
            await apiHelper.addResource({ resource, image });
            await apiHelper.addPipeline({ pipeline, resource, trigger: true });

            await wait.forSuccess(async () => await apiHelper.getCronJob(`${pipeline}-${resource}`));

            const cronJob = await apiHelper.getCronJob(`${pipeline}-${resource}`);

            const expectedEnvironment = [ { name: 'REPO', value: 'git@github.com:username/repo.git' } ];
            const expectedVolumeMounts = [
                {
                    name: 'my-config',
                    mountPath: '/root/.ssh',
                    readOnly: true
                }
            ];
            const expectedVolume = {
                name: 'my-config',
                secret: {
                    defaultMode: 420,
                    secretName: 'my-config',
                    items: [
                        { key: 'id-rsa.pub', path: 'id-rsa.pub' }
                    ]
                }
            };

            expect(manifestHelper.getCronContainers(cronJob).map(c => c.image)).toEqual([ image ]);
            expect(manifestHelper.getCronContainers(cronJob).map(c => c.env)).toEqual([ expectedEnvironment ]);
            expect(manifestHelper.getCronContainers(cronJob).map(c => c.volumeMounts)).toEqual([ expectedVolumeMounts ]);
            expect(cronJob.spec.jobTemplate.spec.template.spec.volumes).toEqual([ expectedVolume ]);
        }, TIMEOUT);

        it('should not add a cronjob to monitor a non-triggering resource for a pipeline', async () => {
            await apiHelper.addResource({ resource, image });
            await apiHelper.addPipeline({ pipeline, resource, trigger: false });

            await wait.forSuccess(async () => {
                await expect(apiHelper.getCronJob(`${pipeline}-${resource}`)).rejects.toEqual(new Error(`cronjobs.batch "${pipeline}-${resource}" not found`));
            });
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