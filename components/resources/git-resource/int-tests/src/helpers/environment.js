module.exports = {
    namespace: process.env['TEST_NAMESPACE'],
    host: process.env['TEST_GIT_HOST'],
    repo: process.env['TEST_REPO'],
    user: process.env['TEST_USER'],
    branch: process.env['TEST_BRANCH'],
    resourceImage: process.env['GIT_RESOURCE_IMAGE'],
    inspectorImage: process.env['FILE_INSPECTOR_IMAGE'],
    sshKey: process.env['TEST_SSH_KEY'],
    sshPublicKey: process.env['TEST_SSH_PUBLIC_KEY']
}