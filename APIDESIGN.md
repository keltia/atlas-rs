# API Design

## Usual way

```rs
let cl = Client::new();

...

let cl = ClientBuilder::new()
            .verbose(true)
            .pool_size(20);
            .build();
...

let p = cl.get_probe(666);

...

let m = cl.NTP(host);     

...

let p = cl.get_probe(666)
            .with(opts)
            .call();       
```

## Another way

```rs
let p = Probe::get(cl, pn)
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


## New possible way (or both?)

TL;DR: probably too complicated as it required the `Value` stuff to be implemented.

```rs
let cl = ClientBuilder::new()
            .verbose(true)
            .pool_size(20)
            .build();
...

let p = Atlas::GetProbe(cl, 666)
            .with(opts)             # with(opts) or with(opt, val) ?
            .call();

...

let m = Atlas::NTP(cl, host).
            .with(opts)
            .call();
            

```
