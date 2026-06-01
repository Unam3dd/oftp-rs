#!/usr/bin/env python3
"""Injecte la section OFTP1 vs OFTP2 dans chaque fiche PDU."""
from pathlib import Path

SECTIONS = {
    "SSRM.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 (RFC 2204) | OFTP2 (RFC 5024) |
|--------|------------------|------------------|
| Présence | ✅ §5.3.1 | ✅ §5.3.1 — **inchangé** |
| Format | `'I'` + `ODETTE FTP READY ` + CR | Identique |
| Implémentation | Même constante octets pour les deux versions | Idem |

Pas de différence de format sur le fil.
""",
    "SSID.md": """
## OFTP1 vs OFTP2

| Champ / aspect | OFTP1 (RFC 2204) | OFTP2 (RFC 5024) |
|----------------|------------------|------------------|
| `SSIDLEV` | Seulement `'1'` | `'1'`,`'2'`,`'4'`,`'5'` (v2.0 = **`5`**) |
| `SSIDSPEC` | Fixe **`N`** (TCP) | **`Y`/`N`** (Y utile X.25 async, pas TCP) |
| `SSIDRSV1` | 5 octets réservés | 4 octets + champ **`SSIDAUTH`** |
| `SSIDAUTH` | ❌ absent | **`Y`/`N`** — auth mutuelle certificats (non négociable) |
| `SSIDCMPR` | Compression **buffer** (§6.2) | Compression **buffer** OFTP (≠ compression **fichier** CMS) |

**Négociation v2** : si `SSIDAUTH` différent entre les deux SSID → **abort session** (pas de compromis).

**Rust** : refuser champs v2 si `SSIDLEV` négocié = 1 ; activer SECD/AUCH/AURP seulement si auth = Y.
""",
    "SFID.md": """
## OFTP1 vs OFTP2

| Champ | OFTP1 | OFTP2 |
|-------|-------|-------|
| `SFIDRSV1` | 9 octets | **3** octets |
| `SFIDDATE` | `YYMMDD` (6) | **`CCYYMMDD` (8)** |
| `SFIDTIME` | `HHMMSS` (6) | **`HHMMSScccc` (10)** |
| `SFIDFSIZ` | 7 chiffres (max ~10⁷ Ko) | **13 chiffres** (fichier transmis) |
| `SFIDOSIZ` | ❌ absent | **13 chiffres** (taille fichier **original** avant CMS) |
| `SFIDREST` | 9 chiffres | **17 chiffres** |
| `SFIDSEC` | ❌ | **00–03** (aucun / chiffré / signé / les deux) |
| `SFIDCIPH` | ❌ | Suite crypto (CMS) |
| `SFIDCOMP` | ❌ | Compression fichier (0=non, 1=ZLIB) |
| `SFIDENV` | ❌ | Enveloppe CMS |
| `SFIDSIGN` | ❌ | EERP signé **Y/N** |
| `SFIDDESC` | ❌ | Description **UTF-8** variable |

**Interop** : un SFID v2 est **plus long** ; un parser v1 ne peut pas recevoir les champs sécurité. Fichier signé/chiffré traité en format **`U`** pour restart même si source F/V.

**Codes SFNA v2** liés : 14–20 (direction, cipher, crypto, compression, signature).
""",
    "SFPA.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Octet commande | `'2'` | `'2'` — identique |
| `SFPAACNT` | **Numeric(9)** | **Numeric(17)** — aligné sur `SFIDREST` v2 |

Sémantique identique : position restart acquiescée ≤ position demandée dans SFID.
""",
    "SFNA.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Structure de base | `SFNACMD` + `SFNAREAS` + `SFNARRTR` | Identique |
| Texte libre | ❌ | **`SFNAREASL` + `SFNAREAST`** (UTF-8, max 999 oct.) |

**Codes raison ajoutés en v2** :

| Code | Signification |
|------|----------------|
| `14` | File direction refused |
| `15` | Cipher suite not supported |
| `16` | Encrypted file not allowed |
| `17` | Unencrypted file not allowed |
| `18` | Compression not allowed |
| `19` | Signed file not allowed |
| `20` | Unsigned file not allowed |

Codes `01`–`13`, `99` : communs aux deux versions.
""",
    "DATA.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| `DATACMD` | `'D'` | `'D'` |
| `DATABUF` | **String(n)** | **Binary(n)** — données binaires après chiffrement/compression |

Sémantique identique : un buffer DATA par exchange buffer, flow control CDT inchangé.
""",
    "CDT.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Format | `'C'` + 2 octets réservés | **Identique** |

Aucune différence de PDU entre RFC 2204 et RFC 5024.
""",
    "EFID.md": """
## OFTP1 vs OFTP2

| Champ | OFTP1 | OFTP2 |
|-------|-------|-------|
| `EFIDRCNT` | Numeric(**9**) | Numeric(**17**) |
| `EFIDUCNT` | Numeric(**12**) | Numeric(**17**) |

Sémantique identique : compteurs **totaux** du fichier (même en restart).
""",
    "EFPA.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Format | `'4'` + `EFPACD` (Y/N) | **Identique** |

Pas de changement de structure entre les deux RFC.
""",
    "EFNA.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Structure | `EFNACMD` + `EFNAREAS` seulement | + **`EFNAREASL` + `EFNAREAST`** (UTF-8) |

**Codes raison EFNA** : v1 s’arrête à `13` et `99`. v2 ajoute **`14`–`23`** :

| Code v2 | Signification |
|---------|----------------|
| `14`–`20` | Comme SFNA (direction, crypto, compression, signature) |
| `21` | Invalid file signature |
| `22` | File decryption failure |
| `23` | File decompression failure |

En OFTP1, l’échec de traitement fichier en fin de transfert n’avait pas ces codes dédiés sur EFNA.
""",
    "ESID.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Structure | `ESIDREAS` + CR | + **`ESIDREASL` + `ESIDREAST`** (UTF-8) |
| Codes `00`–`10`, `99` | ✅ | ✅ (même sens) |
| Code **`11`** | ❌ | Invalid challenge response (auth) |
| Code **`12`** | ❌ | Secure authentication requirements incompatible |

**Rust** : mapper `11`/`12` seulement si session auth CMS active.
""",
    "CD.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Format | 1 octet `'R'` | **Identique** |
| Phases d’usage | Start File, End File, End Session | Identique |

Aucune différence de PDU.
""",
    "EERP.md": """
## OFTP1 vs OFTP2

| Champ | OFTP1 | OFTP2 |
|-------|-------|-------|
| Identité fichier | DSN, date, time, user, dest, orig | Même logique |
| `EERPDATE` / `EERPTIME` | 6 + 6 car. (`YYMMDD`, `HHMMSS`) | **8 + 10** (`CCYYMMDD`, `HHMMSScccc`) |
| `EERPRSV1` | 9 octets | 3 octets |
| `EERPHSH` / `EERPHSHL` | ❌ | Hash fichier **transmis** (si EERP signé) |
| `EERPSIG` / `EERPSIGL` | ❌ | Signature **CMS** optionnelle |

**OFTP1** : accusé positif bout-en-bout **sans** hash ni signature sur le fil.  
**OFTP2** : si `SFIDSIGN=Y`, l’EERP doit être signé (champs hash + signature).

Pas de NERP en v1 : l’échec métier passait par SFNA/EFNA, pas par un accusé négatif E2E.
""",
    "NERP.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Commande | **N’existe pas** | **`N`** — Negative End Response (§5.3.14) |

**Rôle v2** : accusé **négatif** bout-en-bout (échec traitement chez le destinataire final), symétrique de l’EERP.

Champs spécifiques v2 : `NERPCREA` (créateur du NERP), `NERPREAS` + texte UTF-8, hash/signature CMS comme EERP.

**Rust** : ne pas émettre/parser NERP en mode OFTP1 ; tables §9.10 transitions **Y** / **Z1**.
""",
    "RTR.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Format | `'P'` seul | **Identique** |

Flow control après EERP **ou NERP** en v2 — en v1 seulement après EERP (pas de NERP).
""",
    "SECD.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Commande | **Absente** | **`J`** — Security Change Direction |

Introduit avec l’auth mutuelle post-SSID (`SSIDAUTH=Y`). Démarre l’échange AUCH/AURP dans un sens puis l’autre.

**Rust** : module auth uniquement OFTP2 ; ignoré si négociation v1.
""",
    "AUCH.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Commande | **Absente** | **`A`** — Authentication Challenge |

Contient défi aléatoire chiffré **CMS** avec le certificat du pair (lié au `SSIDCODE`).

**Rust** : dépendance CMS + magasin certificats.
""",
    "AURP.md": """
## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| Commande | **Absente** | **`S`** — Authentication Response |

Attention : octet `'S'` = AURP en auth, mais **`SSIDSR='S'`** = Send-only dans SSID (contexte différent).

Réponse au défi AUCH ; vérification signature → `F_CONNECT_CF` ou ESID(11).
""",
}

ROOT = Path(__file__).resolve().parents[1] / "01-rfc5024/05-commandes"
MARKER = "## OFTP1 vs OFTP2"
IMPL = "## Implémentation"

for name, section in SECTIONS.items():
    path = ROOT / name
    if not path.exists():
        print("skip", name)
        continue
    text = path.read_text(encoding="utf-8")
    if MARKER in text:
        # replace existing section up to next ## 
        import re
        text = re.sub(
            r"\n## OFTP1 vs OFTP2\n.*?(?=\n## |\Z)",
            "\n" + section.strip() + "\n\n",
            text,
            count=1,
            flags=re.DOTALL,
        )
    elif IMPL in text:
        text = text.replace(IMPL, section.strip() + "\n\n" + IMPL)
    else:
        text = text.rstrip() + "\n" + section.strip() + "\n"
    path.write_text(text, encoding="utf-8")
    print("ok", name)

print("done")
