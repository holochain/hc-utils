#############################
# █▀█ █▀▀ █░░ █▀▀ ▄▀█ █▀ █▀▀
# █▀▄ ██▄ █▄▄ ██▄ █▀█ ▄█ ██▄
#############################
# requirements
# - cargo-edit crate: `cargo install cargo-edit` >= 0.12.2
# - jq linux terminal tool : `sudo apt-get install jq`
# How to make a update?
#	- Update the version-manager.json file
# 	- make update
#	- make publish
# Publishing

publish:
	git checkout -b v$(shell jq .hdk ./version-manager.json)
	git commit -a -m "version bump $(shell jq .hdk ./version-manager.json)"
	cd ./crates/hc-utils && cargo publish
	git tag $(shell jq .hdk ./version-manager.json)
	git push origin v$(shell jq .hdk ./version-manager.json)
	git push origin refs/tags/$(shell jq .hdk ./version-manager.json)
update:
	echo '⚙️  Updating hdk crate...'
	cargo upgrade -p hdk@=$(shell jq .hdk ./version-manager.json) --pinned
	echo '⚙️  Build...'
	cargo update; cargo build
	echo '⚙️  Version bump of hc_utils crate...'
	cargo set-version $(shell jq .hdk ./version-manager.json) --workspace
	cargo update