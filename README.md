<p align="center">
  <h1 align="center">kjob</h1>
</p>

<p align="center">
  The fastest way to run a job in Kubernetes
</p>

🚧 WIP

## Usage

1. Create a yaml file name `myjob.yaml` with the job configuration.

Example:

```yaml
name: job-name
namespace: my-ns
image: xxx
command: xxxx
```

2. Run the job via:

```sh
kjob -f myjob.yaml > logs.txt
```
