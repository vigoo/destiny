# Development journal of destiny

Created the project and the first component (`destiny:store`) with golem cli.

- Had to do a `git init` and added `.idea` to `.gitignore`
- Renamed `common-lib` to `destiny-model` 
- Defined an initial data model and API for the `store` in WIT
- Changed rust edition to 2024
- Added `#[allow(static_mut_refs)]` to the `mod bindings` to remove annoying warnings 
- Straightforward store implementation with in-memory state
- 