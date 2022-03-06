# OispaHallaAnticheat
[![Rust-CI](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust.yml/badge.svg)](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust.yml)
[![Rust-Release-for-Linux](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust-artifact.yml/badge.svg)](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust-artifact.yml)
[![Rust-Release-for-Windows](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust-artifact-windows.yml/badge.svg)](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust-artifact-windows.yml)
# Lataaminen
Tämä repo käyttää git -submoduuleja, jotka pitää ladata ennen projektin koontia.

Voit ladata ne automaattisesti repon kanssa samaan aikaan käyttämällä komentoa ```git clone --recurse-submodules git@github.com:hallabois/OispaHallaAnticheat.git``` tai suorittamalla komennon ```git pull --recurse-submodules``` aiemmin kloonatussa repossa. 

Submopduulit voi päivittää myöhemmin komennolla ```git submodule update --remote --merge```.
# Koonti ja suorittaminen
Käyttää rustia, asenna se aluksi: https://www.rust-lang.org/tools/install

Huom: webserveri vaatii vieläpä rustin nightly-version, josta voit asentaa tuoreen version komennoilla ```rustup update && rustup default nightly```

Tämän jälkeen serverin voi koota ja aloittaa komennoilla ```cargo build --release && ./target/release/g2048engine --server```

# HTTPS-tuki
```ROCKET_TLS={certs="/etc/letsencrypt/live/hac.hallacoin.ml/fullchain.pem",key="/etc/letsencrypt/live/hac.hallacoin.ml/privkey.pem"} ./target/release/g2048engine --server```

# API:n Käyttö
## /HAC/validate
```
https://hac.oispahalla.com:8000/HAC/validate/[leveys]x[korkeus]S[peli]
---------------------------------------------
{"valid": true, "score": 584, "breaks": 0}
    |              |             |
    |              \-------\     \------------\
    |                      |                  |
Onko syöte validi peli     |   Kuinka monta kurinpalautusta pelin aikana käytettiin (-||-)
                           |
                 Mitä pelin pisteiden tulisi olla (palauttaa nollan, jos peli ei ole validi)
```
e.g. [https://hac.oispahalla.com:8000/HAC/validate/liianpitkätähän](https://hac.oispahalla.com:8000/HAC/validate/4x4S0.0.0.4.0.0.0.0.0.0.2.0.0.0.0.0+1,1.2;1:0.0.0.4.0.2.0.0.0.0.0.2.0.0.0.0+1,1.4;2:0.0.0.0.0.4.0.0.0.0.0.4.0.2.0.2+3,3.2;0:0.4.0.4.0.2.0.2.0.0.0.0.0.0.0.2+3,0.4;3:8.0.0.4.4.0.0.0.0.0.0.0.2.0.0.0+1,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.2.2.0.4+0,3.2;0:8.2.0.4.4.0.0.0.2.0.0.0.2.0.0.0+2,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.4.2.2.4+2,1.2;1:0.0.0.0.0.0.2.8.0.0.0.4.0.4.4.4+1,3.2;0:0.4.2.8.0.0.4.8.0.0.0.0.0.2.0.0+2,1.2;2:0.0.0.0.0.0.2.0.0.4.2.0.0.2.4.16+3,1.2;3:0.0.0.0.2.0.0.2.4.2.0.0.2.4.16.0+0,0.2;1:2.0.0.0.0.0.0.4.0.0.4.2.0.2.4.16+1,2.2;0:2.2.8.4.0.0.0.2.0.2.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.4.0.0.0.2.2.4.8.16+3,0.2;3:2.0.0.2.4.0.0.0.2.0.0.0.2.4.8.16+2,1.2;1:0.0.0.4.0.0.2.4.0.0.0.2.2.4.8.16+1,1.4;0:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+;e:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+1,1.2;0:2.8.2.8.0.2.8.2.0.0.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.8.0.8.2.2.2.2.8.16+3,0.2;3:2.0.0.2.8.0.0.0.8.4.0.0.4.8.16.0+2,1.2;1:0.0.0.4.0.0.2.8.0.0.8.4.0.4.8.16+0,0.2;0:2.4.2.4.0.0.16.8.0.0.0.4.0.0.0.16+0,0.2;2:2.0.0.4.0.0.0.8.0.0.2.4.2.4.16.16+3,0.2;3:2.4.0.2.8.0.0.0.2.4.0.0.2.4.32.0+2,1.2;1:0.2.4.2.0.0.2.8.0.0.2.4.0.2.4.32+0,2.2;0:0.4.4.2.0.0.4.8.2.0.4.4.0.0.0.32+1,1.4;1:0.0.8.2.0.4.4.8.0.0.2.8.0.0.0.32+1,3.2;0:0.4.8.2.0.0.4.16.0.0.2.32.0.2.0.0+1,3.2;3:4.8.2.0.4.16.0.0.2.32.0.0.2.2.0.0+0,3.4;1:0.4.8.2.0.0.4.16.0.0.2.32.4.0.0.4+0,2.2;0:4.4.8.2.0.0.4.16.2.0.2.32.0.0.0.4+2,2.2;3:8.8.2.0.4.16.0.0.4.32.2.0.4.0.0.0+1,3.2;1:0.0.16.2.0.0.4.16.0.4.32.2.0.2.0.4+2,0.2;3:16.2.2.0.4.16.0.0.4.32.2.0.2.4.0.0+3,2.2;2:0.2.0.0.16.16.0.0.8.32.0.2.2.4.4.0+1,0.2;1:0.2.0.2.0.0.0.32.0.8.32.2.0.0.2.8+0,3.2;2:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+;e:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+1,3.2;0:2.2.32.2.0.8.2.32.0.0.0.2.0.2.0.8+3,3.2;3:4.32.2.0.8.2.32.0.2.0.0.0.2.8.0.2+0,3.2;0:4.32.2.2.8.2.32.0.4.8.0.0.2.0.0.0+3,0.2;3:4.32.4.2.8.2.32.0.4.8.0.0.2.0.0.0+1,3.2;1:4.32.4.2.0.8.2.32.0.0.4.8.0.2.0.2+0,3.2;0:4.32.4.2.0.8.2.32.0.2.4.8.2.0.0.2+2,3.2;3:4.32.4.2.8.2.32.0.2.4.8.0.4.0.2.0+0,2.2;1:4.32.4.2.0.8.2.32.2.2.4.8.0.0.4.2+1,0.2;2:0.2.0.2.0.32.4.32.4.8.2.8.2.2.8.2+2,0.4;3:4.0.4.0.32.4.32.0.4.8.2.8.4.8.2.0+0,1.4;1:0.0.0.8.4.32.4.32.4.8.2.8.0.4.8.2+1,0.2;2:0.2.0.8.0.32.4.32.0.8.2.8.8.4.8.2+3,2.2;3:2.8.0.0.32.4.32.0.8.2.8.2.8.4.8.2+1,0.2;1:0.2.2.8.0.32.4.32.8.2.8.2.8.4.8.2+3,0.2;3:4.8.0.2.32.4.32.0.8.2.8.2.8.4.8.2+3,2.2;0:4.8.32.4.32.4.16.2.16.2.0.2.0.4.0.0+0,2.2;1:4.8.32.4.32.4.16.2.2.0.16.4.0.0.0.4+3,3.2;0:4.8.32.4.32.4.32.2.2.0.0.8.0.0.0.2+3,3.2;3:4.8.32.4.32.4.32.2.2.8.0.0.2.0.0.2+1,2.2;1:4.8.32.4.32.4.32.2.0.2.2.8.0.0.0.4+0,3.2;0:4.8.64.4.32.4.2.2.0.2.0.8.2.0.0.4+0,0.2;2:2.0.0.4.4.8.0.2.32.4.64.8.2.2.2.4+3,0.4;3:2.4.0.4.4.8.2.0.32.4.64.8.4.2.4.0+0,1.2;1:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+;e:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+0,2.2;0:2.8.2.8.32.4.8.2.2.0.64.8.0.0.2.4+2,3.2;3:2.8.2.8.32.4.8.2.2.64.8.0.2.4.2.0+0,3.2;1:2.8.2.8.32.4.8.2.0.2.64.8.2.2.4.2+1,3.2;0:2.8.2.8.32.4.8.2.2.4.64.8.0.2.4.2+0,0.2;2:2.0.2.8.2.8.8.2.32.8.64.8.2.2.4.2+3,3.2;3:4.8.0.0.2.16.2.0.32.8.64.8.4.4.2.2+1,0.2;1:0.2.4.8.0.2.16.2.32.8.64.8.0.0.8.4+0,1.2;0:32.4.4.8.2.8.16.2.0.0.64.8.0.0.8.4+0,1.2;2:0.0.4.8.2.0.16.2.32.4.64.8.2.8.8.4+3,3.2;3:4.8.0.0.2.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.0.4.8.2.2.16.2.32.4.64.8.2.16.4.2+3,0.2;3:4.8.0.2.4.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.4.8.2.2.4.16.2.32.4.64.8.2.16.4.2+3,0.2;2:0.0.8.2.2.4.16.4.32.8.64.8.2.16.4.2+1,3.2;0:2.4.8.2.32.8.16.4.2.16.64.8.0.2.4.2+3,3.2;3:2.4.8.2.32.8.16.4.2.16.64.8.2.4.2.2+0,3.2;1:2.4.8.2.32.8.16.4.2.16.64.8.2.2.4.4+0,0.2;2:2.4.8.2.2.8.16.4.32.16.64.8.4.2.4.4+3,3.2;3:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+;e:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+0,0.2;f) tai

[https://hac.oispahalla.com:8000/HAC/validate/4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1](https://hac.oispahalla.com:8000/HAC/validate/4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1)

## /HAC/get_config

```
https://hac.oispahalla.com:8000/HAC/get_config
---------------------------------------------
{
  "allowed_origins": [                                             sallitut CORS-lähteet,
    "http://localhost:8080",                                       eli mistä osoitteista selain saa kutsua api:ta
    "https://oispahalla.com/",
    "http://oispahalla.com",
    "http://oispahalla-dev.netlify.app/",
    "https://oispahalla-dev.netlify.app/",
    "https://dev--oispahalla-dev.netlify.app",
    "https://dev.oispahalla.com/"
  ],
  "platform": "x86_64-unknown-linux-gnu",                          millä alustalla kyseinen instanssi pyörii
  "version": "febc9c91bd18d4be6b4989e3d24898c9bb12ca84",           mikä oli viimeisin git-commit ennen kasaamista
  "rust_version": "rustc 1.57.0-nightly (e1e9319d9 2021-10-14)",   millä rustin versiolla projekti on kasattu
  "request_count": 0                                               kuinka monta kertaa /validate komentoa on kutsuttu
}
```
