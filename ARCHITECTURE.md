# src/

Kansio, joka sisältää kaiken ohjelman lähdekoodin

# target/

Kansio, johon kasattu ohjelma päätyy

# Cargo.toml

Ikäänkuin rustin package.json. Määrittelee ohjelman metadatan, oletusominaisuudet ja tarvitut kirjastot.

# README.md

Lyhyt ohje ohjelman toiminnasta.

# fly.toml

Ohjeet, jonka perusteella fly.io suorittaa ohjelman docker-ympäristössä. Varsinaista dockerin toimintaa ohjaa [Dockerfile](Dockerfile).

# build.rs

Kerää metadataa ohjelman kasauksesta aikana
