# API Design

## Usual way

```rs
let cl = Client::new()
            .verbose(true)
            .pool_size(20);
...

let p = cl.get_probe(666);

...

let m = cl.NTP(host);            
```

## New possible way

```rs
let cl = Client::new()
            .verbose(true)
            .pool_size(20);
...

let p = Atlas::GetProbe(cl, 666)

...

let m = Atlas::NTP(cl, host).opts(...).call()

```
