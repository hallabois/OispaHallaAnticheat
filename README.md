# OispaHallaAnticheat
Käyttää rustia, asenna se aluksi: https://www.rust-lang.org/tools/install

Huom: webserveri vaatii vieläpä rustin nightly-version, josta voit asentaa tuoreen version komennoilla ```rustup update && rustup default nightly```

Tämän jälkeen serverin voi koota ja aloittaa komennoilla ```cargo build --release && ./target/release/g2048engine --server```

# HTTPS
```ROCKET_TLS={certs="/etc/letsencrypt/live/hac.hallacoin.ml/fullchain.pem",key="/etc/letsencrypt/live/hac.hallacoin.ml/privkey.pem"} ./target/release/g2048engine --server```
