ADMIN_KEY := $(shell stellar keys address admin)
OWNER_KEY := $(shell stellar keys address owner)
USER_KEY := $(shell stellar keys address user)
PUBLIC_ADMIN_KEY := GDPDXDW4ZRUSVZO7PDGFEC5UWGXICNPAXXTFQUKTMOINEQRHHOUXWA43

.PHONY: run build test

init-local-stellar:
	podman run --rm -it -p "8000:8000" --name stellar docker.io/stellar/quickstart --testnet

run: 
	pnpm run dev

stellar-admin-key:
	@echo $(ADMIN_KEY)

stellar-owner-key:
	@echo $(OWNER_KEY)

stellar-user-key:
	@echo $(USER_KEY)

stellar-add-public-admin:
	stellar keys add public-admin --secret-key

contract-test:
	cargo test

contract-build:
	rm -rf target/wasm32v1-none/ && cargo build --target wasm32v1-none --release && stellar contract optimize --wasm target/wasm32v1-none/release/rent_a_car.wasm

contract-admin-address:
	@echo $(ADMIN_KEY)

contract-owner-address:
	@echo $(OWNER_KEY)

contract-user-address:
	@echo $(USER_KEY)

stellar-create-admin:
	stellar keys generate admin --network testnet --fund

stellar-create-owner:
	stellar keys generate owner --network testnet --fund

stellar-create-user:
	stellar keys generate user --network testnet --fund

contract-deploy:
	stellar contract deploy \
	--wasm target/wasm32v1-none/release/rent_a_car.optimized.wasm \
	--source admin \
	--network testnet \
	--alias rent_a_car-contract \
	-- \
	--admin $(PUBLIC_ADMIN_KEY) \
	--token CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC

contract-admin-fee-set:
	stellar contract invoke \
	--id rent_a_car-contract \
  	--source public-admin \
  	--network testnet \
  	-- \
  	set_admin_fee \
  	--admin_fee 13

contract-add-car:
	stellar contract invoke \
	--id rent_a_car-contract \
  	--source admin \
  	--network testnet \
  	-- \
  	add_car \
  	--owner $(OWNER_KEY) \
  	--price_per_day 1500

contract-get-car-status:
	stellar contract invoke \
	--id rent_a_car-contract \
	--source admin \
	--network testnet \
	-- \
	get_car_status \
	--owner $(OWNER_KEY)

contract-rental:
	stellar contract invoke \
		--id rent_a_car-contract \
		--source user \
		--network testnet \
		-- \
		rental \
		--renter $(USER_KEY) \
		--owner $(OWNER_KEY) \
		--total_days_to_rent 3 \
		--amount 4500

contract-remove-car:
	stellar contract invoke \
		--id rent_a_car-contract \
		--source admin \
		--network testnet \
		-- \
		remove_car \
		--owner $(OWNER_KEY)

contract-payout-owner:
	stellar contract invoke \
		--id rent_a_car-contract \
		--source owner \
		--network testnet \
		-- \
		payout_owner \
		--owner $(OWNER_KEY) \
		--amount 4500