# API Design

## Usual way

This way has the inconvenient of "polluting" client as everything done through methods
on the `Client`struct (like we do in Go.)

```rs
let cl = Client::new(Config{});

various parameter are set from the `Config` struct

...

let p = cl.get_probe(666);

...

let m = cl.NTP(host);     

...

let p = cl.get_probe(666)
            .with(opts)
            .call();       
```

## Yet another one (like reqwest)

```rs
let c = Client::new("FOO");      // defaults

let c = ClientBuilder::new().key("FOO")
            .verbose()
            .default_probe(666)
            .build();

let p = c.probe().info(666).call();

let pl = c.probe().list(opts).call();

// there c.<category>() returns a RequestBuilder and .call() returns a Response.
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

except

.call() -> reqwest::Response


Atlas API

/api/v2

          ----- /anchor-measurement     ----- /list  ----- List<AM>
                                        ----- /get   ----- AM

          ----- /anchors                ----- /list  ----- List<A>
                                        ----- /get   ----- A

          ----- /credits                ----- /get
                                        ----- /get   ----- /incomes
                                                           /expenses
                                                           /transfers
                                                           /transactions
                                                           /members
                                                           /members      ----- /claim

          ----- /keys                   ----- /permissions
                                        ----- /permissions ----- P     ---- /targets
                                        ----- /get
                                        ----- /set
                                        ----- /delete
                                        ----- /list
                                        ----- /create

          ----- /measurements           ----- /list
                                        ----- /create
                                        ----- /get
                                        ----- /update
                                        ----- /delete

          ----- /participation-requests ----- /list

          ----- /probes                 ----- /get
                                        ----- /list
                                        ----- /set
                                        ----- /update
                                        ----- P             ----- /measurements
                                        ----- /archive
                                        ----- /rankings
                                        ----- /tags
                                        ----- /tags         ----- /slugs

Per context/cmd:

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


