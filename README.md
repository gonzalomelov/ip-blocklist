# IP Blocklist

IP Blocklist provides a microservice that manages a blocklist of IPs. This service would be used to prevent abuse in different applications to ban IPs that are known to be used for malicious purposes.

## Functionalities

The service provides a single REST endpoint that take an IP v4 encoded as a string (e.g. `"127.0.0.1"`), and return `"true"` if the IP is part of the blacklist, and `"false"` otherwise.

This is an example of how calling the microservice can look like:

```bash
$ curl http://blocklist/ips/127.0.0.1
false
```

## Characteristics

- Highy-Available (zero-downtime using [Kubernetes Rollout](https://kubernetes.io/docs/concepts/workloads/controllers/deployment/#use-case) when updating the blocklist).
- Blazingly fast and memory-efficient using [Rust](https://www.rust-lang.org/) and its renown web framework [actix-web](https://www.techempower.com/benchmarks/#section=data-r21) to tackle a performance-critical service like validating IPs is.
- In sync with [this public list](https://github.com/stamparm/ipsum) of suspicious and/or malicious IP addresses.

## What's Included?

This repository contains:

- The implementation of the microservice.
- Instructions on how to install, test, and run it.
- An explanation of the design choices made, and how it meets both the functional and non-functional requirements.
- An explanation of any compromise or trade-off took because of time constraints.

## Source code

Please refer to:
- [/src](/src) Microservice implementation
- [/tests](/tests) E2E tests
- [/scripts](/scripts) Deployment scripts

## Quick start

### Install & Run

The first things you need to do are cloning this repository, installing its
dependencies and running the app:

```sh
git clone https://github.com/gonzalomelov/ip-blocklist.git
cd ip-blocklist
cargo run
```

Finally, to verify that it's working properly, open a terminal and execute:

```sh
curl 0.0.0.0:8080/ips/95.214.24.192
```

### Test

Execute the following command to test the endpoint:

```sh
cargo test
```

## Kubernetes Deployment

The complete system has 2 main deployments:

- IP Blocklist: API serving the unique GET method to check if the IP is part of the blacklist

- CronJob: Executes once a day, ideally after [this public list](https://github.com/stamparm/ipsum) is updated. Initially it generates a new API image with the updated data. Afterwards it pushes the image to Docker Hub to have it ready to update the cluster. Finally, a rollout is executed where new pods are created and replaces the outdated ones incrementally.

### Install IP Blocklist

```sh
kubectl create -f k8s-ip-blocklist.yml
```

### Install CronJob

```sh
kubectl create secret docker-registry regcred --docker-server=https://index.docker.io/v1/ --docker-username=$USERNAME --docker-password=$PASSWORD --docker-email=$EMAIL
kubectl apply -f k8s-cronjob.yaml
```

### Execute CronJob manually

Execute the following instruction in order to test the functionality right away, and thus execute the job asap:

```sh
kubectl create job --from=cronjob/restart-deployment restart-deployment-001
```

## Troubleshoot or rollback

### Install IP Blocklist

```sh
kubectl get pods
kubectl delete -f k8s-ip-blocklist.yml
```

### Install IP Blocklist -> Check or delete

```sh
kubectl get pods
kubectl delete -f k8s-ip-blocklist.yml
```

### Install CronJob -> Check or delete

```sh
kubectl apply -f k8s-cronjob.yaml
```

### Execute CronJob manually -> Check and debug

```sh
kubectl logs job.batch/restart-deployment-001
kubectl get pod restart-deployment-001-5dzl5 --template '{{.status.initContainerStatuses}}'
kubectl logs restart-deployment-001-5dzl5 -c kaniko-demo
```

### Check downloaded ipsum file -> Check and debug

```sh
kubectl get pods
kubectl exec --stdin --tty ip-blocklist-867777c8bc-dxn9r -- /bin/bash
cat /usr/src/ip-blocklist/ips.csv
curl 0.0.0.0:8080/ips/95.214.24.192
```

### Build, run and push image manually

```sh
docker build --pull --no-cache -t ip-blocklist:$version .
docker tag ip-blocklist:$version gonzalomelov17/ip-blocklist:$version
docker run -it -p 8080:8080 --rm --name ip-blocklist ip-blocklist:$version
docker push gonzalomelov17/ip-blocklist:$version
```

### Kubernetes Rollout manually

```sh
kubectl set image deployment.apps/ip-blocklist image=gonzalomelov17/ip-blocklist:$version
kubectl annotate deployment.apps/ip-blocklist kubernetes.io/change-cause="ipsum updated to $version"
kubectl rollout status deployment.apps/ip-blocklist
kubectl rollout history deployment.apps/ip-blocklist
``` 

## Architecture

The most important aspects that needed to be tackled from the ground up were:
1. Performant API due to the app characteristics

First and foremost, the API must be fast because it'll be used under heavy load. After a quick research, in which I found some [benchmarks](https://www.techempower.com/benchmarks/#section=data-r21) (didn't validate them due to time restrictions), I decided to go with Rust.

It's important to mention that **I didn't have any experience developing using the language**, but as this aspect is the most important one, I decided to take the risky move because the app wasn't so complex and the time spent learning and trying, wasn't going to be extensive.

2. Code simplicity

Firstly, the application only needs to serve one endpoint so there is no need to add unnecesary extensive libraries that are not going to be fully used and that will add extra hidden processing steps to the normal processing of the requests. However, in order to speedup the development of the http-server and make use of other out-of-the-box features (intuitive definitions, testing tools, reasonable community size), I opted to use **actix-web**, one of the renown frameworks in the Rust community. 

Secondly, specifically related to the execution of the app:
- Initializes reading the already updated file from storage and storing the data in-memory using a HashSet
- Starts the http-server
- When a GET request is received, it checks the indexed HashSet in O(1) and returns if the IP is in the list or not

3. In sync with data source

The CronJob executes once a day, ideally after [this public list](https://github.com/stamparm/ipsum) is updated. Initially it generates a new API image with the updated data. Afterwards it pushes the image to Docker Hub to have it ready to update the cluster. Finally, a rollout is executed where new pods are created and replaces the outdated ones incrementally.

This process usually takes 10 min locally to execute and uses Kaniko for buidling Docker images inside a Kubernetes cluster.

4. This service should be highly available, minimizing the time it takes to restart it and the downtime when updating the blocklist.

The process that updates the app with the new blocklist is separated to the one serving the API. Using Kubernetes Rollout there is no downtime, thus the normal operations of the cluser is not affected.

5. The service should remain operational under heavy load, and be able to respond in a reasonably low time.

Using a simple http-server with a lightweight framework in a performant runtime make the service reliable and sustainable. The Kubernetes Cluster is configured initially with 3 replicas of the App pod and a LoadBalancer sits in front of them.

6. Time constraints

In order to develop and deploy a fully fledged microservice a lot of considerations must be taken. Although the application is developed to be fast, performant tests must be executed to validate the assumptions.

## Compromises or trade-offs

- Validate online benchmarks and run performance tests to decide which language, frameworks or simpler solutions (hyper) to use
- Compare different data structure to support larger IPs list. Right now the API uses a HashSet to index and replies in O(1) with aprox. 100k entries. If the list size increases exponentially a new data structure or another data storage component may be needed to support such a heavy storage.
- Separate Dockerfile for development
- Unit tests: File reader
- Benchmark to another solutions: i.e. Using a database
- Speedup the image creation after the ipsum repository is updated in order to reduce the GAP where the app is not replying with the newly updated IP blocklist