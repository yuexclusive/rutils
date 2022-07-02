#!/bin/zsh

# istio
kind create cluster --config=kube.yaml

istioctl operator init --hub=docker.io/querycapistio --tag=1.13.3

istioctl install -f ./install-istio-arm.yaml -y

# kubectl label namespace default istio-injection=enabled --overwrite

# kubectl label namespace default istio-injection-

# sudo kubectl port-forward -n istio-system service/istio-ingressgateway 80

# cert-manager
# kubectl apply -f https://github.com/jetstack/cert-manager/releases/latest/download/cert-manager.yaml
