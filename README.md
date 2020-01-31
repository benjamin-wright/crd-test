# K8s CRD-based continuous integration service

## Custom Resources

### Task

```yaml
apiVersion: "minion.ponglehub.com/v1"
kind: Task
metadata:
  name: my-task
spec:
  pipeline: parent-pipeline
  image: task-image
  inputs:
   - input1
   - input2
```