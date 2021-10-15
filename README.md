# OispaHallaAnticheat
Käyttää rustia, asenna se aluksi: https://www.rust-lang.org/tools/install

Huom: webserveri vaatii vieläpä rustin nightly-version, josta voit asentaa tuoreen version komennoilla ```rustup update && rustup default nightly```

Tämän jälkeen serverin voi koota ja aloittaa komennoilla ```cargo build --release && ./target/release/g2048engine --server```

# HTTPS
```ROCKET_TLS={certs="/path/to/certs.pem",key="/path/to/key.pem"} ./target/release/g2048engine --server```
