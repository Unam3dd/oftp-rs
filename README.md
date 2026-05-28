# oftp-rs

Implémentation Rust du protocole ODETTE-FTP 2.0 (RFC 5024) — POC handshake + transfert fichier.

## Architecture

```text
oftp-client / oftp-server
       ↓
  OftpSession (TCP, handshake, transfert)
       ↓
  stream (STH + OEB)  |  commands (SSRM, SSID, SFID, DATA, …)
```

## Prérequis

- Rust 2024 edition (`cargo` récent)

## Lancer le POC

Terminal 1 — serveur :

```bash
cargo run --bin oftp-server
```

Terminal 2 — client (handshake seul) :

```bash
cargo run --bin oftp-client -- 127.0.0.1:3305
```

Envoi d'un fichier :

```bash
echo "contenu test" > /tmp/test.txt
cargo run --bin oftp-client -- 127.0.0.1:3305 --file /tmp/test.txt
```

Le serveur enregistre le fichier dans le répertoire courant (option `--output-dir`).

## Logs

```bash
RUST_LOG=oftp_rs=debug cargo run --bin oftp-client -- 127.0.0.1:3305 --file /tmp/test.txt
```

## Tests

```bash
cargo test
```

## Périmètre actuel

- Handshake initiateur / répondeur (SSRM, SSID)
- Transfert fichier : SFID → SFPA → DATA (+ CDT) → EFID → EFPA → **EERP → RTR**
- Fin de session (ESID)
- Modules PDU : SFPA, SFNA, DATA, CDT, EFID, EFPA, EFNA, CD, EERP, RTR, ESID

Non implémenté : sécurité (SECD/AUCH), compression, restart, NERP, table RFC complète.

## Interop mendelson

Si mendelson affiche **« Attente de confirmation »**, c’était en général l’absence d’**EERP** après réception du fichier — corrigé dans `run_receive_file`.

Checklist côté mendelson (partenaire vers `oftp-server`) :

| Paramètre | Valeur typique |
|-----------|----------------|
| Protocole | OFTP 2.0 (niveau 5) |
| Hôte | `127.0.0.1` (ou IP du serveur) |
| Port | `3305` (défaut du binaire) |
| SSIDCODE | identique à `--ssid-code` du serveur |
| Mot de passe | identique à `--password` si utilisé |
| TLS / chiffrement | désactivé (comme le POC) |

Le serveur doit tourner **avant** l’envoi mendelson :

```bash
RUST_LOG=oftp_rs=debug cargo run --bin oftp-server -- --ssid-code "VOTRE_CODE_ODETTE"
```

Les fichiers reçus apparaissent dans le répertoire courant (ou `--output-dir`).

### Problèmes mendelson fréquents (d’après les logs)

| Symptôme dans mendelson | Cause | Action |
|-------------------------|-------|--------|
| `ODETTE id not known` | SSIDCODE inconnu | Créer le partenaire avec le même code que `--ssid-code` (`O01779122072341` → partenaire **Stales Business**) |
| `Invalid filename` / minuscules dans SFIDDSN | DSN en minuscules | Le client convertit le nom en **MAJUSCULES** (`rfc5024.txt` → `RFC5024.TXT`) |
| `0 Byte reçu` | Fichier source vide | Envoyer un fichier non vide : `dd if=/dev/zero bs=1024 count=1 of=/tmp/TEST` |
| `Attente EERP` + `127.0.0.1:3305/<unresolved>:3305` | mendelson tente une **connexion sortante** vers une adresse invalide (souvent vers lui-même) | Dans le partenaire **Stales Business** : corriger hôte/port d’appel (ex. `127.0.0.1` + port où écoute `oftp-server`, ex. `3306`), supprimer tout hostname `<unresolved>`. Garder la session TCP ouverte côté client jusqu’à EERP+RTR. |

Exemple d’envoi vers mendelson (port **3305** = mendelson écoute) :

```bash
dd if=/dev/zero bs=1024 count=1 of=/tmp/TEST
RUST_LOG=oftp_rs=debug cargo run --bin oftp-client -- 127.0.0.1:3305 --file /tmp/TEST --ssid-code "O01779122072341"
```
