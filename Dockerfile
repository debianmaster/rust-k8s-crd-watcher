FROM scratch
COPY target/release/rust-k8s-crd-watcher /rust-k8s-crd-watcher
CMD ['/rust-k8s-crd-watcher']
