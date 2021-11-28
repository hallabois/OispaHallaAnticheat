# OispaHallaAnticheat
# Lataaminen
Tämä repo käyttää git -submoduuleja, jotka pitää ladata ennen projektin koontia.

Voit ladata ne automaattisesti repon kanssa samaan aikaan käyttämällä komentoa ```git clone --recurse-submodules git@github.com:hallabois/OispaHallaAnticheat.git``` tai suorittamalla komennon ```git pull --recurse-submodules``` aiemmin kloonatussa repossa. 

Submopduulit voi päivittää myöhemmin komennolla ```git submodule update```.
# Koonti ja suorittaminen
Käyttää rustia, asenna se aluksi: https://www.rust-lang.org/tools/install

Huom: webserveri vaatii vieläpä rustin nightly-version, josta voit asentaa tuoreen version komennoilla ```rustup update && rustup default nightly```

Tämän jälkeen serverin voi koota ja aloittaa komennoilla ```cargo build --release && ./target/release/g2048engine --server```

# HTTPS-tuki
```ROCKET_TLS={certs="/etc/letsencrypt/live/hac.hallacoin.ml/fullchain.pem",key="/etc/letsencrypt/live/hac.hallacoin.ml/privkey.pem"} ./target/release/g2048engine --server```

# API:n Käyttö
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
e.g. [https://hac.oispahalla.com:8000/HAC/validate/liianpitkätähän](https://hac.oispahalla.com:8000/HAC/validate/4x4S0.0.0.4.0.0.0.0.0.0.2.0.0.0.0.0+1,1.2;1:0.0.0.4.0.2.0.0.0.0.0.2.0.0.0.0+1,1.4;2:0.0.0.0.0.4.0.0.0.0.0.4.0.2.0.2+3,3.2;0:0.4.0.4.0.2.0.2.0.0.0.0.0.0.0.2+3,0.4;3:8.0.0.4.4.0.0.0.0.0.0.0.2.0.0.0+1,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.2.2.0.4+0,3.2;0:8.2.0.4.4.0.0.0.2.0.0.0.2.0.0.0+2,3.2;2:0.0.0.0.8.0.0.0.4.0.0.0.4.2.2.4+2,1.2;1:0.0.0.0.0.0.2.8.0.0.0.4.0.4.4.4+1,3.2;0:0.4.2.8.0.0.4.8.0.0.0.0.0.2.0.0+2,1.2;2:0.0.0.0.0.0.2.0.0.4.2.0.0.2.4.16+3,1.2;3:0.0.0.0.2.0.0.2.4.2.0.0.2.4.16.0+0,0.2;1:2.0.0.0.0.0.0.4.0.0.4.2.0.2.4.16+1,2.2;0:2.2.8.4.0.0.0.2.0.2.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.4.0.0.0.2.2.4.8.16+3,0.2;3:2.0.0.2.4.0.0.0.2.0.0.0.2.4.8.16+2,1.2;1:0.0.0.4.0.0.2.4.0.0.0.2.2.4.8.16+1,1.4;0:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+;e:2.4.2.8.0.4.8.2.0.0.0.16.0.0.0.0+1,1.2;0:2.8.2.8.0.2.8.2.0.0.0.16.0.0.0.0+1,0.2;2:0.2.0.0.0.0.0.8.0.8.2.2.2.2.8.16+3,0.2;3:2.0.0.2.8.0.0.0.8.4.0.0.4.8.16.0+2,1.2;1:0.0.0.4.0.0.2.8.0.0.8.4.0.4.8.16+0,0.2;0:2.4.2.4.0.0.16.8.0.0.0.4.0.0.0.16+0,0.2;2:2.0.0.4.0.0.0.8.0.0.2.4.2.4.16.16+3,0.2;3:2.4.0.2.8.0.0.0.2.4.0.0.2.4.32.0+2,1.2;1:0.2.4.2.0.0.2.8.0.0.2.4.0.2.4.32+0,2.2;0:0.4.4.2.0.0.4.8.2.0.4.4.0.0.0.32+1,1.4;1:0.0.8.2.0.4.4.8.0.0.2.8.0.0.0.32+1,3.2;0:0.4.8.2.0.0.4.16.0.0.2.32.0.2.0.0+1,3.2;3:4.8.2.0.4.16.0.0.2.32.0.0.2.2.0.0+0,3.4;1:0.4.8.2.0.0.4.16.0.0.2.32.4.0.0.4+0,2.2;0:4.4.8.2.0.0.4.16.2.0.2.32.0.0.0.4+2,2.2;3:8.8.2.0.4.16.0.0.4.32.2.0.4.0.0.0+1,3.2;1:0.0.16.2.0.0.4.16.0.4.32.2.0.2.0.4+2,0.2;3:16.2.2.0.4.16.0.0.4.32.2.0.2.4.0.0+3,2.2;2:0.2.0.0.16.16.0.0.8.32.0.2.2.4.4.0+1,0.2;1:0.2.0.2.0.0.0.32.0.8.32.2.0.0.2.8+0,3.2;2:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+;e:0.0.0.2.0.0.0.32.0.2.32.2.2.8.2.8+1,3.2;0:2.2.32.2.0.8.2.32.0.0.0.2.0.2.0.8+3,3.2;3:4.32.2.0.8.2.32.0.2.0.0.0.2.8.0.2+0,3.2;0:4.32.2.2.8.2.32.0.4.8.0.0.2.0.0.0+3,0.2;3:4.32.4.2.8.2.32.0.4.8.0.0.2.0.0.0+1,3.2;1:4.32.4.2.0.8.2.32.0.0.4.8.0.2.0.2+0,3.2;0:4.32.4.2.0.8.2.32.0.2.4.8.2.0.0.2+2,3.2;3:4.32.4.2.8.2.32.0.2.4.8.0.4.0.2.0+0,2.2;1:4.32.4.2.0.8.2.32.2.2.4.8.0.0.4.2+1,0.2;2:0.2.0.2.0.32.4.32.4.8.2.8.2.2.8.2+2,0.4;3:4.0.4.0.32.4.32.0.4.8.2.8.4.8.2.0+0,1.4;1:0.0.0.8.4.32.4.32.4.8.2.8.0.4.8.2+1,0.2;2:0.2.0.8.0.32.4.32.0.8.2.8.8.4.8.2+3,2.2;3:2.8.0.0.32.4.32.0.8.2.8.2.8.4.8.2+1,0.2;1:0.2.2.8.0.32.4.32.8.2.8.2.8.4.8.2+3,0.2;3:4.8.0.2.32.4.32.0.8.2.8.2.8.4.8.2+3,2.2;0:4.8.32.4.32.4.16.2.16.2.0.2.0.4.0.0+0,2.2;1:4.8.32.4.32.4.16.2.2.0.16.4.0.0.0.4+3,3.2;0:4.8.32.4.32.4.32.2.2.0.0.8.0.0.0.2+3,3.2;3:4.8.32.4.32.4.32.2.2.8.0.0.2.0.0.2+1,2.2;1:4.8.32.4.32.4.32.2.0.2.2.8.0.0.0.4+0,3.2;0:4.8.64.4.32.4.2.2.0.2.0.8.2.0.0.4+0,0.2;2:2.0.0.4.4.8.0.2.32.4.64.8.2.2.2.4+3,0.4;3:2.4.0.4.4.8.2.0.32.4.64.8.4.2.4.0+0,1.2;1:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+;e:0.0.2.8.2.4.8.2.32.4.64.8.0.4.2.4+0,2.2;0:2.8.2.8.32.4.8.2.2.0.64.8.0.0.2.4+2,3.2;3:2.8.2.8.32.4.8.2.2.64.8.0.2.4.2.0+0,3.2;1:2.8.2.8.32.4.8.2.0.2.64.8.2.2.4.2+1,3.2;0:2.8.2.8.32.4.8.2.2.4.64.8.0.2.4.2+0,0.2;2:2.0.2.8.2.8.8.2.32.8.64.8.2.2.4.2+3,3.2;3:4.8.0.0.2.16.2.0.32.8.64.8.4.4.2.2+1,0.2;1:0.2.4.8.0.2.16.2.32.8.64.8.0.0.8.4+0,1.2;0:32.4.4.8.2.8.16.2.0.0.64.8.0.0.8.4+0,1.2;2:0.0.4.8.2.0.16.2.32.4.64.8.2.8.8.4+3,3.2;3:4.8.0.0.2.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.0.4.8.2.2.16.2.32.4.64.8.2.16.4.2+3,0.2;3:4.8.0.2.4.16.2.0.32.4.64.8.2.16.4.2+0,1.2;1:0.4.8.2.2.4.16.2.32.4.64.8.2.16.4.2+3,0.2;2:0.0.8.2.2.4.16.4.32.8.64.8.2.16.4.2+1,3.2;0:2.4.8.2.32.8.16.4.2.16.64.8.0.2.4.2+3,3.2;3:2.4.8.2.32.8.16.4.2.16.64.8.2.4.2.2+0,3.2;1:2.4.8.2.32.8.16.4.2.16.64.8.2.2.4.4+0,0.2;2:2.4.8.2.2.8.16.4.32.16.64.8.4.2.4.4+3,3.2;3:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+;e:2.4.8.2.2.8.16.4.32.16.64.8.4.2.8.2+0,0.2;f)
