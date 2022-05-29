# API Design

## Usual way (aka feature `flat_api`)

This way has the inconvenient of "polluting" client as everything done through methods
on the `Client`struct (like we do in Go.).  Next issue is passing options (same thing as in `ripe-atlas`).

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

let p = c.probe().get(666).call();

let pl = c.probe().list(opts).call();     // or c.probe().list().with(opts).call();

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

### List of operations per category

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

    client.rs                  anchor.rs/.../probe.rs           request.rs

    c = Client::new()
    c = ClientBuilder::new()

    c.anchor()
    c.anchor_measurement()
    c.credits()
    c.keys()
    c.probe()
                                                                RequestBuilder()
                                                                get(N)
                                                                list()
                                                                info()

                                  <Type>::dispatch()
                                                                 with(opts)

                                                                .call()


    struct Callable<T> {
    // ...
    }
    impl<T> Callable<T> {
    pub fn call(self) -> T {
    todo!()
    }
    }
    
    fn get<T>() -> Callable<T> {
    Callable { /* ... */ }
    }
    
    fn list<T>() -> Callable<Vec<T>> {
    Callable { /* ... */ }
    }
    