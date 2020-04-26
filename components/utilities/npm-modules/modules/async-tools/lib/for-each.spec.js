const forEach = require('./for-each');

describe('forEach', () => {
    it('should work', async () => {
        const data = [1, 2, 3];
        const fn = jest.fn(x => Promise.resolve(x));

        await forEach(data, async value => await fn(value));

        expect(fn.mock.calls).toEqual([[1], [2], [3]]);
    });
});