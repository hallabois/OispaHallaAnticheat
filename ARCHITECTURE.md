# src/
Kansio, joka sisältää kaiken ohjelman lähdekoodin
# docs/
Generoitu dokumentaatio, kts. [#rebuild_docs.sh](#rebuild_docs.sh)
# target/
Kansio, johon kasattu ohjelma päätyy
# twothousand-forty-eight/
Tarvittu kirjasto git-submoduulina, lue [README.md](README.md).
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
# Rocket.toml
Ohjaa Rocket-palvelinkirjaston toimintaa
# build.rs
Kerää metadataa ohjelman kasauksesta aikana