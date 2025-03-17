# Development journal of destiny

Created the project and the first component (`destiny:store`) with golem cli.

- Had to do a `git init` and added `.idea` to `.gitignore`
- Renamed `common-lib` to `destiny-model` 
- Defined an initial data model and API for the `store` in WIT
- Added `#[allow(static_mut_refs)]` to the `mod bindings` to remove annoying warnings
- Straightforward store implementation with in-memory state, some improvements of the WIT spec on the way

Checked the authentication page in the docs to understand how it will work. As we don't have an API definition yet, there was no immediate action items but understood that the result will be an `email` string available for invocations.

Based on this, came up with the following initial architecture:

- `destiny:store` - The store component. Each worker name will consist of `email/store-name` 
- `destiny:user` - The user component. Each worker belongs to a registered user, identified by email

We add an an `email` parameter to each store operation that will be filled by the logged-in user's token. We also have to store a list of allowed users in the store state.
Decided to represent this change by introducing a resource where the user is the constructor and move all exported functions to that. Did this change in WIT and code.

Had to introduce an "initialize" global export that can be only called once, and sets the initial owner.
Had to change all result types to include error so we can indicate access denied.

As a next step, created a new component `destiny:user`. Starting with it's WIT.

While writing the initial implementation, realized that it is going to need to call the store `initialize` so we need RPC. 
So added the dependency to the root golem.yaml.

To implement it we need to resolve worker id from owner email and store name. To keep things typesafe (even though they are strings now) decided to move the `user` wit type to a common WIT file and generate the bindings in the shared `destiny_model` crate.

With that set implemented calling `initialize` using RPC.
