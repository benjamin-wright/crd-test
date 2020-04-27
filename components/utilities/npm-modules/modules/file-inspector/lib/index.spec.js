jest.mock('./request');

const request = require('./request');
const FileInspector = require('./index');

describe('file-inspector', () => {
    beforeEach(() => {
        jest.resetAllMocks();

        this.inspector = new FileInspector('name', 'namespace');
    });

    describe('list', () => {
        [
            { it: 'should error for 404 response', code: 404, errorMessage: 'Error getting files list: 404' },
            { it: 'should error for 500 response', code: 500, errorMessage: 'Error getting files list: 500' },
            { it: 'should error for 503 response', code: 503, errorMessage: 'Error getting files list: 503' }
        ].forEach(options => {
            it(options.it, async () => {
                request.call.mockResolvedValue({ status: options.code });
                await expect(async() => this.inspector.list()).rejects.toEqual(new Error(options.errorMessage));
            });
        });

        it('should call the right url', async () => {
            request.call.mockResolvedValue({ status: 200 });

            await this.inspector.list();

            expect(request.call).toHaveBeenCalledWith('http://name.namespace.svc.cluster.local/list', 'GET');
        });

        it('should return the response data string', async () => {
            request.call.mockResolvedValue({ status: 200, data: 'list data' });

            const list = await this.inspector.list();

            expect(list).toEqual('list data');
        });
    });

    describe('get', () => {
        [
            { it: 'should error for 404 response', code: 404, errorMessage: 'Error getting file "filename": 404' },
            { it: 'should error for 500 response', code: 500, errorMessage: 'Error getting file "filename": 500' },
            { it: 'should error for 503 response', code: 503, errorMessage: 'Error getting file "filename": 503' }
        ].forEach(options => {
            it(options.it, async () => {
                request.call.mockResolvedValue({ status: options.code });
                await expect(async() => this.inspector.get('filename')).rejects.toEqual(new Error(options.errorMessage));
            });
        });

        it('should call the right url', async () => {
            request.call.mockResolvedValue({ status: 200 });

            await this.inspector.get('filename');

            expect(request.call).toHaveBeenCalledWith('http://name.namespace.svc.cluster.local/file/filename', 'GET');
        });

        it('should return the response data string', async () => {
            request.call.mockResolvedValue({ status: 200, data: 'file data' });

            const list = await this.inspector.get('filename');

            expect(list).toEqual('file data');
        });
    });

    describe('exit', () => {
        [
            { it: 'should error for 404 response', code: 404, errorMessage: 'Error shutting down the file-inspector: 404' },
            { it: 'should error for 500 response', code: 500, errorMessage: 'Error shutting down the file-inspector: 500' },
            { it: 'should error for 503 response', code: 503, errorMessage: 'Error shutting down the file-inspector: 503' }
        ].forEach(options => {
            it(options.it, async () => {
                request.call.mockResolvedValue({ status: options.code });
                await expect(async() => this.inspector.exit()).rejects.toEqual(new Error(options.errorMessage));
            });
        });

        it('should call the right url', async () => {
            request.call.mockResolvedValue({ status: 202 });

            await this.inspector.exit();

            expect(request.call).toHaveBeenCalledWith('http://name.namespace.svc.cluster.local/exit', 'POST');
        });

        it('should return the response data string', async () => {
            request.call.mockResolvedValue({ status: 202, data: 'file data' });

            const list = await this.inspector.exit();

            expect(list).toEqual('file data');
        });
    });
});