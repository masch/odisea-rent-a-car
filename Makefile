.PHONY: run build test

init-local-stellar:
	podman run --rm -it -p "8000:8000" --name stellar docker.io/stellar/quickstart --testnet

run: 
	pnpm run dev

