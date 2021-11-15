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

## New possible way (or both?)

TL;DR: probably too complicated as it required the `Value` implementation to be working.

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
