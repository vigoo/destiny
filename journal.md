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

At this point, before we figure out how to create API mappings and UI, let's try to write _integration tests_ for the store and user components.
Plan:
- start local golem or assume it is running
- use the user and store components directly through the invocation API
- implement it as rust tests, use teh `golem-client` rust crate for communication

After writing the first integration test, realized that:
**Invalid worker name: Worker name must contain only alphanumeric characters, underscores, and dashes**

Problem: we cannot encode it differently because we get the email from the auth token and we have to encode it with Rib!

We do a workaround:
- introducing a singleton component `destiny:accounts` that will do the mapping from email to user name (valid worker name)
- use `__` instead of `/` in the store worker name as separator

With these changes the first integration test passes.


Next let's figure out how we want to develop the frontend. I want to try Dioxus. The idea is that we use Dioxus in frontend-only mode (not full stack as we have our backend as workers), and pack the UI files (html and wasm and assets) into a new worker called `destiny:ui` as initial file system. This component will be ephemeral and the API gateway will route to its static files.

Let's try to set this up.

`cargo install dioxus-cli`

Creating a new dioxus app separately first with the `dx` tool to see how it is set up. Then we convert it to a golem component.

Also a new `destiny:ui` rust component. Copy the barebones dioxus app to it. `dx serve` works in the `components-rust/destiny-ui` dir.
To package it we run `dx bundle`. Let's integrate this into the app manifest.

After some experiments with dioxus, let's try to make the first two endpoints with the api gateway.

New issues!
- We need to know the deployed component ID and we the cli does not print it now. But we can get it from the log (for now)
- Bigger problem: the `destiny:accounts` trick will not work because we cannot call two different components from a single endpoint's rib script

For the second we can do an even bigger hack to workaround it:
we can export a proxy for the accounts api in the `destiny:user` component, and spawn an ephemeral instance from the rib script
to call it.
We don't set up the auth yet so the script will have
```rib
let email = "user@test.com";
```
instead.

NOTE: In commit e6327a50e6d27d045be1625d3df744f7151e98f3 the api yaml produces very weird rib parse error.

Registering and deploying the api

```
golem api definition new api.yaml
golem api definition update api.yaml
golem api deployment deploy destiny-api-v1/0.0.1 --host localhost:9006
```

First try with curl, we get
```
curl -v localhost:9006/api/stores

Failed to map input type Str(TypeStr) to any of the expected content types: "\"*/*\""
```
`-H "Accept: application/json"` fixes it

We cannot try this out at the moment from `dx serve` because of CORS. Can we fix that?
Yes by adding a new route. Needs to delete deployment, register a new definition and deploy it again.

As a next step, let's serve the UI through the API gateway.

First we need to change the golem:ui component type to ephemeral. Then we change the dioxus routing to 
have a `/ui` prefix because we cannot define at the moment bindings for `/` in the api gateway.
Then we bind `/ui</*>` to `/web/public/index.html` and `/*` to `/web/public/$1` on the ephemeral ui worker.


Adding more UI implementation.
Turns out if we add API to `destiny:store` we are going to need to add a accounts proxy there too.

Then ran into a weird error:

```rib
        let email = "user@test.com";
        let temp_worker = instance("__accounts_proxy");
        let user = temp_worker.get-user-name(email);
        let store_name = request.path.store;
        let store_worker_name = "${user}__${store_name}";
        let worker = instance(store_worker_name);
        let store = worker.store(user);
        let result = store.get-currency();
        
        match result {
          ok(currency) => { status: 200u64, body: currency },
          err(error) => { status: 404u64, body: "Failed to get currency: ${error}" }
        }
```

gives 

```
error: API Definition Service - Error: 500 Internal Server Error, Rib internal error: Global variables not allowed: user. Allowed: request
```

no idea why?
