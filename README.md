# Fake-vestalis

## Objectifs

Ce repository a deux objectifs principaux:
- Permettre de tester eo-connect en avance de phase des développements par Vestalis (branche `testing`)
- Servir d'implémentation de référence pour Vestalis (branche `main`)

## Prérequis

### Rust

Ce code a été écrit en rust 1.69.0 et doit donc être installé soit manuellement via [`rustup`](https://rustup.rs/), soit via nix en utilisant le fichier `shell.nix` fournit.

### Variables d'environnement

Trois variables d'environnement doivent être présente pour pouvoir lancer le serveur :

- SIGNATURE_KEY doit contenir le chemin vers la clé privée utilisé pour la signature.
- CIPHER_KEY doit contenir la clé de chiffrement partagée entre Eove et Vestalis.
- CONNECT_NAME doit contenir le nom du serveur eo-connect à utiliser.

### Clé de signature

Les clés de signature ed25519 peuvent être générées via :
```shell
bash ./scripts/generate-ed25519.sh
```
Elles seront alors disponible dans `security/keys`.

### Certificat ssl pour https

La communication avec le serveur est sécurisée via HTTPS.
Il faut donc créer une authorité et un certificat pour le serveur.
La commande :
```shell
bash ./scripts/generate-ca.sh
```
Crée une autorité dans le  dossier `security/ca`.

La commande :
```shell
bash ./scripts/generate-server-certificate.sh
```
Crée un certificat pour le serveur pour qu'il tourne sur localhost dans le dossier `security/server`.
Si le nom de domaine souhaité est différent, alors il faut créer un fichier `.ext` en s'inspirant de `scripts/vestalis.ext` et renseigner la variable d'environnement `SERVER_CERTIFICATE_EXTENSION_PATH` pour contenir le chemin vers ce fichier.
La commande précédent créera alors un certificat adapté.

Enfin, si le certificat et/ou l'autorité ne sont pas crées dans les dossiers mentionnés, alors il faudra mettre à jour le fichier `Rocket.toml` en fonction.

## Lancer le server

Pour lancer le serveur, lancer la commande suivante :
```shell
cargo run
```
Le serveur sera alors accessible sur le port `8042`.

## Lancer les tests

L'exemple décrit par les spécifications a été implémenté en tant que test.
Le test peut être lancé via :
```shell
cargo test
```
