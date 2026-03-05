# `pubchem-rs`
<img src="https://pride-badges.pony.workers.dev/static/v1?label=trans%20rights&stripeWidth=6&stripeColors=5BCEFA,F5A9B8,FFFFFF,F5A9B8,5BCEFA" alt="Trans Rights"/>
<br/>
Rust client for Pubchem REST API, inspired by [`pubchem`](https://docs.rs/pubchem/latest/pubchem/)

---

# Usage
Create an instance of `Client` to query the API.
### Get compound ID by formula
```rust
use pubchem_rs::client::Client;

let mut client = Client::new();
client.cid_by_formula("H2O").await.unwrap();
```
---
### Get compound by CID
```rust
use pubchem_rs::client::Client;

let client = Client::new();
client.compound_by_cid(962).await.unwrap();
```
---
### Get compound ID by formula
```rust
use pubchem_rs::client::Client;

let mut client = Client::new();
client.compound_by_formula("MgO").await.unwrap();
```