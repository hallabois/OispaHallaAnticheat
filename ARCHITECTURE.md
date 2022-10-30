# src/
Kansio, joka sisältää kaiken ohjelman lähdekoodin
# docs/
Generoitu dokumentaatio, kts. [#rebuild_docs.sh](#rebuild_docs.sh)
# target/
Kansio, johon kasattu ohjelma päätyy
# Cargo.toml
Ikäänkuin rustin package.json. Määrittelee ohjelman metadatan, oletusominaisuudet ja tarvitut kirjastot.
# README.md
Lyhyt ohje tämän ohjelman toiminnasta. 
Älä muokkaa suoraan, sillä tämä generoidaan automaattisesti tiedoston src/main.rs kommenteista, 
kts. [#rebuild_docs.sh](#rebuild_docs.sh) ja [#README.tpl](#README.tpl)
# README.tpl
Tiedosto, jonka avulla README.md kootaan
# rebuild_docs.sh
Skripti joka generoi kaiken dokumentaation (docs/) ja README.md:n uudelllen
# fly.toml
Ohjeet, jonka perusteella fly.io suorittaa ohjelman docker-ympäristössä. Varsinaista dockerin toimintaa ohjaa [Dockerfile](Dockerfile).
# build.rs
Kerää metadataa ohjelman kasauksesta aikana
