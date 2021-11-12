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

## New possible way

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
