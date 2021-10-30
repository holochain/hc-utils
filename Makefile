#############################
# █▀█ █▀▀ █░░ █▀▀ ▄▀█ █▀ █▀▀
# █▀▄ ██▄ █▄▄ ██▄ █▀█ ▄█ ██▄
#############################
# requirements
# - cargo-edit crate: `cargo install cargo-edit`
# - jq linux terminal tool : `sudo apt-get install jq`
# How to make a update?
#	- Update the version-manager.json file
# 	- make update
#	- make publish
# Publishing

publish:
	cd ./crates/hc-utils && cargo publish

update:
	echo '⚙️  Updating hdk crate...'
	cargo upgrade hdk@=$(shell jq .hdk ./version-manager.json) --workspace
	echo '⚙️  Updating holo_hash crate...'
	cargo upgrade holo_hash@=$(shell jq .holo_hash ./version-manager.json) --workspace
	echo '⚙️  Build...'
	cargo update; cargo build
	echo '⚙️  Version bump of hc_utils crate...'
	cargo set-version $(shell jq .hdk ./version-manager.json) --workspace
