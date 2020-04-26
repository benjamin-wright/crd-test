const sleep = require('./sleep');
jest.useFakeTimers();

describe('sleep', () => {
    it('should not resolve after 29 seconds', async () => {
        const fn1 = jest.fn();
        const fn2 = jest.fn();

        (async () => {
            fn1();
            await sleep(30);
            fn2();
        })();

        expect(fn1).toHaveBeenCalled();
        expect(fn2).not.toHaveBeenCalled();

        await Promise.resolve();
        await jest.advanceTimersByTime(29);
        await Promise.resolve();

        expect(fn2).not.toHaveBeenCalled();

        await jest.advanceTimersByTime(31);
        await Promise.resolve();
    });
    it('should resolve after 30 seconds', async () => {
        const fn1 = jest.fn();
        const fn2 = jest.fn();

        (async () => {
            fn1();
            await sleep(30);
            fn2();
        })();

        expect(fn1).toHaveBeenCalled();
        expect(fn2).not.toHaveBeenCalled();

        await Promise.resolve();
        await jest.advanceTimersByTime(30);
        await Promise.resolve();

        expect(fn2).toHaveBeenCalled();
    });
});