# OispaHallaAnticheat

[![Rust-CI](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust.yml/badge.svg)](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust.yml)
[![Rust-Release-for-Linux](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust-artifact.yml/badge.svg)](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust-artifact.yml)
[![Rust-Release-for-Windows](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust-artifact-windows.yml/badge.svg)](https://github.com/hallabois/OispaHallaAnticheat/actions/workflows/rust-artifact-windows.yml)

Vilpinestopalvelin peliin 2048 pohjautuvalle [oispahalla:lle](https://oispahalla.com/), pohjautuu kirjastoon [twothousand-forty-eight](https://github.com/hallabois/twothousand-forty-eight).

## Koonti ja suorittaminen

Käyttää rustia, asenna se aluksi: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

Huom: palvelin-ominaisuus vaatii vieläpä rustin nightly-version, josta voit asentaa tuoreen version komennoilla `rustup update && rustup default nightly`

## HTTPS-tuki

aja palvelin komennolla `TLS_CERT="[sertifikaatti]" TLS_KEY="[avain]" ./target/release/OispaHallaAnticheat --server`

## Projektin Tiedostorakenne

Lue [ARCHITECTURE.md](ARCHITECTURE.md).

## API:n Käyttö

### /api/validate

```rust
https://hac.oispahalla.com/api/validate/[leveys]x[korkeus]S[peli]
---------------------------------------------
{
    "run_hash":"724...",   Pelin uniikki tunniste. Älä käytä, jos peli ei ole validi
    "board_w":4,           Tulkittu pelilaudan leveys
    "board_h":4,           Tulkittu pelilaudan korkeus
    "valid":true,          Onko syöte validi peli
    "score":584,           Mitä pelin pisteiden tulisi olla*
    "score_margin":4,      Viimeisen vuoron aiheuttama mahdollinen pisteiden liikkumisvara*
    "breaks":0,            Kuinka monta kurinpalautusta pelin aikana käytettiin*
    "length":85            Pelin pituus siirroissa
}
*palauttaa nollan, jos peli ei ole validi
```

e.g. [https://hac.oispahalla.com/api/validate/liianpitkätähän](https://hac.oispahalla.com/api/validate/4x4S0.0.0.4.0.0.0.0.0.0.2.0.0.0.0.0+1,1.2;1:0.0.0.4.0.2.0.0.0.0.0.2.0.0.0.0+1,1.4;2:0.0.0.0.0.4.0.0.0.0.0.4.0.2.0.2+3,3.2;0:0.4.0.4.0.2.0.2.0.0.0.0.0.0.0.2+3,0.4;3:8.0.0.4.4.0.0.0.0.0.0.0.2.0.0.0+1,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.2.2.0.4+0,3.2;0:8.2.0.4.4.0.0.0.2.0.0.0.2.0.0.0+2,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.4.2.2.4+2,1.2;1:0.0.0.0.0.0.2.8.0.0.0.4.0.4.4.4+1,3.2;0:0.4.2.8.0.0.4.8.0.0.0.0.0.2.0.0+2,1.2;2:0.0.0.0.0.0.2.0.0.4.2.0.0.2.4.16+3,1.2;3:0.0.0.0.2.0.0.2.4.2.0.0.2.4.16.0+0,0.2;1:2.0.0.0.0.0.0.4.0.0.4.2.0.2.4.16+1,2.2;0:2.2.8.4.0.0.0.2.0.2.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.4.0.0.0.2.2.4.8.16+3,0.2;3:2.0.0.2.4.0.0.0.2.0.0.0.2.4.8.16+2,1.2;1:0.0.0.4.0.0.2.4.0.0.0.2.2.4.8.16+1,1.4;0:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+;e:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+1,1.2;0:2.8.2.8.0.2.8.2.0.0.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.8.0.8.2.2.2.2.8.16+3,0.2;3:2.0.0.2.8.0.0.0.8.4.0.0.4.8.16.0+2,1.2;1:0.0.0.4.0.0.2.8.0.0.8.4.0.4.8.16+0,0.2;0:2.4.2.4.0.0.16.8.0.0.0.4.0.0.0.16+0,0.2;2:2.0.0.4.0.0.0.8.0.0.2.4.2.4.16.16+3,0.2;3:2.4.0.2.8.0.0.0.2.4.0.0.2.4.32.0+2,1.2;1:0.2.4.2.0.0.2.8.0.0.2.4.0.2.4.32+0,2.2;0:0.4.4.2.0.0.4.8.2.0.4.4.0.0.0.32+1,1.4;1:0.0.8.2.0.4.4.8.0.0.2.8.0.0.0.32+1,3.2;0:0.4.8.2.0.0.4.16.0.0.2.32.0.2.0.0+1,3.2;3:4.8.2.0.4.16.0.0.2.32.0.0.2.2.0.0+0,3.4;1:0.4.8.2.0.0.4.16.0.0.2.32.4.0.0.4+0,2.2;0:4.4.8.2.0.0.4.16.2.0.2.32.0.0.0.4+2,2.2;3:8.8.2.0.4.16.0.0.4.32.2.0.4.0.0.0+1,3.2;1:0.0.16.2.0.0.4.16.0.4.32.2.0.2.0.4+2,0.2;3:16.2.2.0.4.16.0.0.4.32.2.0.2.4.0.0+3,2.2;2:0.2.0.0.16.16.0.0.8.32.0.2.2.4.4.0+1,0.2;1:0.2.0.2.0.0.0.32.0.8.32.2.0.0.2.8+0,3.2;2:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+;e:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+1,3.2;0:2.2.32.2.0.8.2.32.0.0.0.2.0.2.0.8+3,3.2;3:4.32.2.0.8.2.32.0.2.0.0.0.2.8.0.2+0,3.2;0:4.32.2.2.8.2.32.0.4.8.0.0.2.0.0.0+3,0.2;3:4.32.4.2.8.2.32.0.4.8.0.0.2.0.0.0+1,3.2;1:4.32.4.2.0.8.2.32.0.0.4.8.0.2.0.2+0,3.2;0:4.32.4.2.0.8.2.32.0.2.4.8.2.0.0.2+2,3.2;3:4.32.4.2.8.2.32.0.2.4.8.0.4.0.2.0+0,2.2;1:4.32.4.2.0.8.2.32.2.2.4.8.0.0.4.2+1,0.2;2:0.2.0.2.0.32.4.32.4.8.2.8.2.2.8.2+2,0.4;3:4.0.4.0.32.4.32.0.4.8.2.8.4.8.2.0+0,1.4;1:0.0.0.8.4.32.4.32.4.8.2.8.0.4.8.2+1,0.2;2:0.2.0.8.0.32.4.32.0.8.2.8.8.4.8.2+3,2.2;3:2.8.0.0.32.4.32.0.8.2.8.2.8.4.8.2+1,0.2;1:0.2.2.8.0.32.4.32.8.2.8.2.8.4.8.2+3,0.2;3:4.8.0.2.32.4.32.0.8.2.8.2.8.4.8.2+3,2.2;0:4.8.32.4.32.4.16.2.16.2.0.2.0.4.0.0+0,2.2;1:4.8.32.4.32.4.16.2.2.0.16.4.0.0.0.4+3,3.2;0:4.8.32.4.32.4.32.2.2.0.0.8.0.0.0.2+3,3.2;3:4.8.32.4.32.4.32.2.2.8.0.0.2.0.0.2+1,2.2;1:4.8.32.4.32.4.32.2.0.2.2.8.0.0.0.4+0,3.2;0:4.8.64.4.32.4.2.2.0.2.0.8.2.0.0.4+0,0.2;2:2.0.0.4.4.8.0.2.32.4.64.8.2.2.2.4+3,0.4;3:2.4.0.4.4.8.2.0.32.4.64.8.4.2.4.0+0,1.2;1:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+;e:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+0,2.2;0:2.8.2.8.32.4.8.2.2.0.64.8.0.0.2.4+2,3.2;3:2.8.2.8.32.4.8.2.2.64.8.0.2.4.2.0+0,3.2;1:2.8.2.8.32.4.8.2.0.2.64.8.2.2.4.2+1,3.2;0:2.8.2.8.32.4.8.2.2.4.64.8.0.2.4.2+0,0.2;2:2.0.2.8.2.8.8.2.32.8.64.8.2.2.4.2+3,3.2;3:4.8.0.0.2.16.2.0.32.8.64.8.4.4.2.2+1,0.2;1:0.2.4.8.0.2.16.2.32.8.64.8.0.0.8.4+0,1.2;0:32.4.4.8.2.8.16.2.0.0.64.8.0.0.8.4+0,1.2;2:0.0.4.8.2.0.16.2.32.4.64.8.2.8.8.4+3,3.2;3:4.8.0.0.2.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.0.4.8.2.2.16.2.32.4.64.8.2.16.4.2+3,0.2;3:4.8.0.2.4.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.4.8.2.2.4.16.2.32.4.64.8.2.16.4.2+3,0.2;2:0.0.8.2.2.4.16.4.32.8.64.8.2.16.4.2+1,3.2;0:2.4.8.2.32.8.16.4.2.16.64.8.0.2.4.2+3,3.2;3:2.4.8.2.32.8.16.4.2.16.64.8.2.4.2.2+0,3.2;1:2.4.8.2.32.8.16.4.2.16.64.8.2.2.4.4+0,0.2;2:2.4.8.2.2.8.16.4.32.16.64.8.4.2.4.4+3,3.2;3:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+;e:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+0,0.2;f) tai
minimi-pituinen peli
[https://hac.oispahalla.com/api/validate/4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1](https://hac.oispahalla.com/api/validate/4x4S0.0.0.0.0.0.0.0.0.0.0.0.0.0.2.2+2,1.2;1)

### /api/get_config

```rust
https://hac.oispahalla.com/api/get_config
---------------------------------------------
{
  "platform": "x86_64-unknown-linux-gnu",                          millä alustalla kyseinen instanssi pyörii
  "version": "febc9c91bd18d4be6b4989e3d24898c9bb12ca84",           mikä oli viimeisin julkaistu versio tai git-commit ennen kasaamista
  "rust_version": "rustc 1.57.0-nightly (e1e9319d9 2021-10-14)",   millä rustin versiolla projekti on kasattu
  "request_count": 0                                               kuinka monta kertaa /validate komentoa on kutsuttu
}
```

[License](LICENSE): MIT
