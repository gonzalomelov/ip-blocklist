# version="latest"
# version=$(date +%F)
# version=$(date +%F_%H-%M-%S)

# Install IP Blocklist
# kubectl create -f k8s-ip-blocklist.yml
# kubectl get pods
# kubectl delete -f k8s-ip-blocklist.yml

# Install CronJob
# kubectl create secret docker-registry regcred --docker-server=https://index.docker.io/v1/ --docker-username=$USERNAME --docker-password=$PASSWORD --docker-email=$EMAIL
# kubectl apply -f k8s-cronjob.yaml

# Execute CronJob manually
# kubectl create job --from=cronjob/restart-deployment restart-deployment-001
# kubectl logs job.batch/restart-deployment-001
# kubectl get pod restart-deployment-001-5dzl5 --template '{{.status.initContainerStatuses}}'
# kubectl logs restart-deployment-001-5dzl5 -c kaniko-demo

# Check downloaded ipsum file
# kubectl get pods
# kubectl exec --stdin --tty ip-blocklist-867777c8bc-dxn9r -- /bin/bash
# cat /usr/src/ip-blocklist/ips.csv
# curl 0.0.0.0:8080/ips/95.214.24.192

# Build, run and push image manually
# docker build --pull --no-cache -t ip-blocklist:$version .
# docker tag ip-blocklist:$version gonzalomelov17/ip-blocklist:$version
# docker run -it -p 8080:8080 --rm --name ip-blocklist ip-blocklist:$version
# docker push gonzalomelov17/ip-blocklist:$version

# Kubernetes Rollout manually
# kubectl set image deployment.apps/ip-blocklist image=gonzalomelov17/ip-blocklist:$version
# kubectl annotate deployment.apps/ip-blocklist kubernetes.io/change-cause="ipsum updated to $version"
# kubectl rollout status deployment.apps/ip-blocklist
# kubectl rollout history deployment.apps/ip-blocklist