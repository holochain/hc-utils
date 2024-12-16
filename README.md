# hc-utils

A util crate for holochain

> Note: Currently the hc-utils crate will always follow the version of hdk, when hdk version is 1.2.1, hc_utils version will be v1.2.1
> Note: all the functions are converted to macros

## Helper Functions

### commit_idempotent

    Query for an existing Entry in the local source-chain matching the given EntryType name(s). If one exists, return it Address, otherwise commit it.

### create_idempotent_link

    Query for an existing Link in the local source-chain matching the given LinkType name(s).  If one exists, return it Address, otherwise commit it.

### exists

    Query for an existing Entry in the local source-chain matching the given EntryType name(s).  If one exists, return it Address, otherwise returns error

### get_action

    Gets the action of an Entry

### get_latest_link

    gets latest link created to the specific base

### local_source_chain

    Returns a list of elements from the local source-chain

### remove_link

    Removes link. The current hdk expects you to get all links and delete_link by specifying the entry_hash of the entry that you want to delete

### wrappers

    String wrapper for all holo_hash types

### get_details

    Get details for a list of links passed using the HDK::borrow functions

### get

    Get element for a list of links passed using the HDK::borrow functions

### get_links_and_load_type

    Gets the entries that are linked to a base with LinkTag by matching with the declared TryFrom Entry.

**Map versions:**

| hc-utils | hdk      |
| -------- | -------- |
| v0.0.107 | v0.0.107 |
| v0.0.110 | v0.0.110 |
| ... | ... |
| v0.0.111 | v0.0.111 |
| v0.0.115 | v0.0.115 |

[![License: CAL 1.0](https://img.shields.io/badge/License-CAL%201.0-blue.svg)](https://github.com/holochain/cryptographic-autonomy-license)

Copyright (C) 2017-2020, Holo Ltd.

This program is free software: you can redistribute it and/or modify it under the terms of the license
provided in the LICENSE file (CAL-1.0). This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
