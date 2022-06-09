# API Design

## Chosen approach (like reqwest)

We use the Builder pattern for Client through ClientBuilder and partially with RequestBuider.

We will have

    Client::new() -> Client (with reasonable defaults)
    Client::builder() -> ClientBuilder

    ClientBuilder::new() -> ClientBuilder
    (all methods)
    .foo(...) -> ClientBuilder

except

    .build() -> Client

then

    Client.probe()          -> RequestBuilder
          .measurement()    -> RequestBuilder
    ...
    RequestBuilder() -> RequestBuilder

    get/info/delete/etc.() 

Let rollback and reintroduce call().

We have two kind of returned data:

- Single

  Information about a single data item

  - Paged

    Information about a list of data items, de-paginated and returned as a single vector
    (or possibly an iterator).

In `RequestBuilder` we need the following:

- Client (it carries configuration and HTTP client)
- `reqwest::Client` (for the calls and to reuse it during pagination for example)

Both support the Callable trait and implement call().

    Client.(first) -> RequestBuilder -> get(P)  -> Single -> with(O)   -> call()
                                                          -> subcmd()  -> with(O) -> call()
                                        list(Q) -> Paged -> with(O)    -> call()
                                                         -> subcmd()  -> with(O) -> call()
                                        info() -> Single -> with(O) -> Single
                                               -> subcmd() -> with(O) -> Single
                                               -> subcmd() -> with(O) -> Paged

```rs
let c = Client::new("FOO");      // defaults

let c = ClientBuilder::new().key("FOO")
            .verbose()
            .default_probe(666)
            .build();

let p = c.probe().get(666);

let pl = c.probe().list(opts);     // or c.probe().list().with(opts);

// there c.<category>() returns a RequestBuilder and .get/info/etc. returns a Response.
```

### List of operations per category

    ----- /anchor-measurement       ----- /list ----- List<AM>
                                    ----- /get ----- AM
    ----- /anchors                  ----- /list ----- List<A>
                                    ----- /get ----- A
    ----- /credits                  ----- /info                                             **DONE**
                                    ----- /get ----- /incomes
                                                     /expenses
                                                     /transfers
                                          /list      /transactions
                                                     /members
                                                     /members ----- /claim
    ----- /keys                     ----- /permissions
                                    ----- /permissions ----- P ---- /targets
                                    ----- /get                                              **DONE**
                                    ----- /set
                                    ----- /delete
                                    ----- /list
                                    ----- /create
    ----- /measurements             ----- /list
                                    ----- /create
                                    ----- /get
                                    ----- /update
                                    ----- /delete
    ----- /participation-requests   ----- /list
    ----- /probes                   ----- /get                                              **DONE**
                                    ----- /list                                             **DONE**
                                    ----- /set
                                    ----- /update
                                    ----- P ----- /measurements
                                    ----- /archive
                                    ----- /rankings
                                    ----- /tags
                                    ----- /tags ----- /slugs

### Per context/cmd:

    RequestBuilder
            list         anchor-measurements/anchors/credits/keys/measurements/participation-requests/probes
            get          anchor-measurements/anchors/credits/keys/measurements/probes
            info         credits
            set          keys/probes
            measurements probes
            permissions  keys
            delete       keys/measurements            
            create       keys/measurements
            update       measurements/probes
            archive      probes
            rankings     probes
            tags         probes
            targets      keys

### Call tree

        c = Client::new()
        c = ClientBuilder::new()
                .api_key("FOO")
                .want_af(AF::6)
                .build()
    ...
        c.anchor()
        c.anchor_measurement()
        c.credits()
        c.keys()
        c.partitipation_requests()
        c.probe()
                 RequestBuilder()
                           get(N)
                           list(Q)
                           info()
                           delete(T)
                           create(T)
                                          with(k, v)
                                          with([(k1,v1),(k2,v2)]
                                                                  call()
        