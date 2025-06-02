.PHONY: bootstrap

bootstrap:
	git config --global --add safe.directory $(shell pwd)/
	apt-get update && apt-get install -y --no-install-recommends gh device-tree-compiler && apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/*
	gh release download spike-1.1.1 --repo LayerResearch/jolt-riscv-arch-test --pattern "spike-1.1.1-Linux-aarch64.tar.gz" -O /tmp/spike.tar.gz && tar -xzf /tmp/spike.tar.gz -C /usr/local/bin/ && rm -rf /tmp/*
