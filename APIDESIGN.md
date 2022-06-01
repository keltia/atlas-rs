# API Design

## Chosen approach (like reqwest)

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

We will have

Client::new() -> Client (with reasonable defaults)
Client::builder() -> ClientBuilder

ClientBuilder::new() -> ClientBuilder
(all methods)
.foo(...) -> ClientBuilder

except

.build() -> Client

then

Client.probe()      -> RequestBuilder
      .measurement()
...

RequestBuilder() -> RequestBuilder

get/info/delete/etc.() -> reqwest::Response

Atlas API

/api/v2

### List of operations per category

    ----- /anchor-measurement       ----- /list ----- List<AM>
                                    ----- /get ----- AM
    ----- /anchors                  ----- /list ----- List<A>
                                    ----- /get ----- A
    ----- /credits                  ----- /info                                             **DONE**
                                    ----- /get ----- /incomes
                                                     /expenses
                                                     /transfers
                                                     /transactions
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
            list        anchor-measurements/anchors/keys/measurements/participation-requests/probes
            get         anchor-measurements/anchors/keys/measurements/probes
            info        credits
            set         keys/probes
            permissions keys
            delete      keys/measurements            
            create      keys/measurements
            update      measurements/probes
            archive     probes
            rankings    probes
            tags        probes

### Call tree

    client.rs anchor.rs/.../probe.rs request.rs
    ....
        c = Client::new()
        c = ClientBuilder::new()
    .....
        c.anchor()
        c.anchor_measurement()
        c.credits()
        c.keys()
        c.partitipation_requests()
        c.probe()
                                    RequestBuilder()
                                                        opt(k, v)
                                                        opts([(k1,v1),(k2,v2)]
                                                                                get(N)
                                                                                list(Q)
                                                                                info()
                                                                                delete()
                                                                                create()
