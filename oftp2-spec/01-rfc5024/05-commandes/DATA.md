---
groupe: commandes
rfc: "5024"
section: "5.3.6"
tags: [rfc5024, pdu, groupe/commandes]
---

# DATA — Data Exchange Buffer

| Propriété | Valeur |
|-----------|--------|
| **Octet 0** | `'D'` |
| **Phase** | Data Transfer |
| **Direction** | **Speaker → Listener** |
| **Tables §9** | Speaker **M** ; Listener **I** |

## Structure PDU (§5.3.6)

| Pos | Champ | Description |
|-----|-------|-------------|
| 0 | DATACMD | `'D'` |
| 1 | DATABUF | Corps §7 : enchaînement **HDR + subrecord** (binaire en v2) |

Un **Exchange Buffer** = **une** commande ; jamais mélanger DATA et une autre PDU dans le même buffer ([[../05-commandes-index]]).

## Taille négociée (`V.Buf-size`)

Valeur issue de **SSID** (`SSIDSDEB`) : **minimum** des deux annonces → `V.Buf-size`.

| Élément | Dans la limite `SSIDSDEB` / `V.Buf-size` ? | Sur le fil TCP (§8) ? |
|---------|---------------------------------------------|------------------------|
| Octet `'D'` + `DATABUF` (OEB = **DEB**) | **Oui** — tout l’OEB ≤ `Buf-size` | **Oui** — à l’intérieur du STB |
| **STH** (4 o, longueur STB) | **Non** — négociation §5.3.2 / SSID | **Oui** — **devant chaque** DEB, DATA inclus |
| **Special logic** (annexe C) | **Non** | Selon profil (souvent N sur TCP/TLS) |

`V.Buf-size` borne l’**OEB** (exchange buffer OFTP), pas le **STB**. Sur TCP : `write(TCP) = STH + OEB` avec `Length` = `4 + len(OEB)` ([[../07-08-buffers#§8 Stream Transmission Buffer (STB)]]). Sans STB sur TCP, le pair ne retrouve pas les frontières des buffers (DATA comme SSID).

Formule de base (DEB seulement) :

```
DATABUF_max = V.Buf-size - 1
```

**Ne pas** lire un fichier par tranches de `Buf-size` octets : le fichier passe par des **subrecords** dans `DATABUF` (§7).

## Format §7 — subrecords dans `DATABUF`

```
'D' | [HDR][payload] [HDR][payload] ...
     └──────────── DATABUF ────────────┘
```

### Octet HDR (1 octet)

| Bits | Nom | Rôle |
|------|-----|------|
| 0 | EOR | Fin d’enregistrement (sur fichier non structuré ≈ fin de fichier) |
| 1 | CF | `0` = littéral · `1` = run d’un octet répété (voir ci-dessous) |
| 2–7 | COUNT | Si CF=`0` : longueur du bloc suivant (0–63) · Si CF=`1` : nombre de répétitions (0–63) |

Chaque subrecord **non compressé** (CF=`0`) coûte **`1 + n`** octets dans `DATABUF` pour **`n`** octets utiles (`n ≤ 63`).

Règles de remplissage : [[../07-08-buffers]] · détail §7.3 (subrecord jamais coupé entre deux buffers ; stratégies de bourrage).

## Compression buffer (bit CF) — §7.1

La RFC décrit un *« simple compression scheme for strings of **repeated characters** »*. Ce n’est **pas** zlib ni la compression fichier CMS : c’est un **RLE minimal** au niveau de chaque subrecord dans `DATABUF`.

### Deux mécanismes à ne pas confondre

| Mécanisme | Où | Négociation | Effet |
|-----------|-----|-------------|--------|
| **Compression buffer** (bit **CF**) | Dans chaque PDU **DATA** (§7) | [[SSID]] `SSIDCMPR` = **Y** des **deux** côtés → `V.Compression` | Répéter **un** octet identique jusqu’à 63 fois |
| **Compression fichier** | Avant OFTP (enveloppe, CMS, zlib…) | [[SFID]] `SFIDCOMP` | Compresse **tout le fichier** ; §6.4 dit que cela **remplace** en pratique la compression buffer (conservée pour compatibilité OFTP1) |

En OFTP2 moderne, beaucoup de stacks n’utilisent que **CF=0** sur le fil et compressent (si besoin) **avant** le mapping §7 via `SFIDCOMP`.

### Subrecord non compressé (CF = 0)

```
HDR (EOR, CF=0, COUNT=n)  |  b₁ b₂ … bₙ
         1 o               |      n o
```

- **COUNT** = nombre d’octets **littéraux** qui suivent dans `DATABUF`.
- Octets fichier reconstruits = copie directe de `b₁…bₙ`.

Exemple : `"ABC"` → HDR (COUNT=3) + `'A' 'B' 'C'` → **4** octets dans `DATABUF` pour **3** octets fichier.

### Subrecord compressé (CF = 1) — « 1 octet × COUNT »

```
HDR (EOR, CF=1, COUNT=n)  |  V
         1 o               |  1 o seulement
```

- Le payload n’est **qu’un seul** octet **V** (pas une séquence).
- **COUNT** = combien de fois écrire **V** dans le **fichier virtuel** à la réception (pas la longueur du bloc sur le fil).

**Décompression (récepteur) :**

```text
si CF = 1 :
    lire V (1 octet après HDR)
    répéter V exactement COUNT fois dans le fichier
sinon :
    lire COUNT octets littéraux
```

Exemple : **30** espaces `0x20` dans le fichier.

| Mode | Sur le fil (`DATABUF`) | Fichier reconstruit |
|------|------------------------|---------------------|
| CF=0 | HDR + 30 × `0x20` → **31** o | 30 espaces |
| CF=1 | HDR (COUNT=30) + `0x20` → **2** o | 30 espaces |

Run > 63 octets : enchaîner plusieurs subrecords CF=1 (ex. COUNT=63 puis COUNT=30 pour 93 octets identiques).

### Rapport avec le RLE « classique »

| | RLE usuel (images, etc.) | OFTP (CF=1) |
|---|--------------------------|-------------|
| Idée | Compacter des répétitions | Oui, cas très restreint |
| Motif | Souvent 1 octet, parfois une **séquence** | **Un seul** octet **V** |
| Max run | Variable | **63** (6 bits COUNT) |
| Sur le fil | `[marqueur][count][données…]` | **HDR + 1 octet** |
| Texte varié (`"ABAB"`) | Parfois encodable | **Non** — rester en CF=0 ou compresser le fichier (SFID) |

Ce n’est **pas** LZ77, **pas** deflate/zlib : uniquement `VVV…` avec le même **V**.

### Quand encoder CF=1 (émetteur)

1. Session : `SSIDCMPR=Y` **et** pair aussi → sinon **toujours CF=0**.
2. Détecter une suite de **≥ 2** (souvent **≥ 3**) octets **identiques** consécutifs dans le fichier.
3. Émettre HDR avec **CF=1**, COUNT = longueur de la suite (≤ 63), puis **V**.
4. Gain typique : **2** octets sur le fil pour jusqu’à **63** octets fichier (utile pour padding, zéros, espaces).

**Seuil de gain** (ordre de grandeur) : CF=0 coûte `1+n` pour `n` octets ; CF=1 coûte **2** pour jusqu’à **63** octets → intéressant dès `n ≥ 3` pour une run pure.

### Impact sur la capacité fichier / PDU

Les tableaux [[#Capacité fichier par PDU DATA]] supposent des subrecords **CF=0** pleins (63 o fichier + 1 HDR = 64 o). Avec **CF=1**, un même `DATABUF` peut représenter **plus** d’octets fichier (jusqu’à 63 × nombre de subrecords compressés), mais le calcul n’est plus une formule fixe — il dépend du contenu (runs répétées).

## Capacité fichier par PDU DATA

### Méthode de calcul

1. `B = V.Buf-size` (ex. 4096).
2. `DATABUF_max = B - 1` (ex. 4095).
3. Subrecords **pleins** (`n = 63`) : chacun occupe **64** octets dans `DATABUF` → **63** octets fichier.
4. `q = DATABUF_max ÷ 64` (partie entière), reste `r = DATABUF_max mod 64`.
5. Fichier des `q` pleins : `q × 63`.
6. S’il reste `r ≥ 2` : un subrecord final avec **`r - 1`** octets de fichier (1 HDR + au plus `r-1` données).
7. **Total fichier** = `q × 63 + max(0, r - 1)` (si `r = 1`, seulement un HDR vide possible, +0 fichier).

### Exemples

| `V.Buf-size` | `DATABUF_max` | Subrecords (remplissage optimal) | Octets **fichier** max / PDU |
|--------------|---------------|----------------------------------|------------------------------|
| 2048 | 2047 | 32 (31× plein + 1 partiel) | **2015** |
| 4096 | 4095 | 64 (63× plein + 1 partiel) | **4031** |

Détail pour **4096** :

```
4095 ÷ 64 = 63 reste 63

63 subrecords pleins : 63 × 64 = 4032 o dans DATABUF → 63 × 63 = 3969 o fichier
reste 63 o           : 1 HDR + 62 o données          → +62 o fichier

Total DATABUF : 4032 + 63 = 4095 ✓
Total fichier : 3969 + 62 = 4031
```

Pourquoi pas **4095** octets de fichier ? Chaque tranche de 63 o « coûte » 64 o dans `DATABUF` à cause du HDR. Sur 4096, la « perte » est **65** o = **1** (`'D'`) + **64** HDR (un par subrecord).

| Cas extrême | Subrecords / PDU | Fichier / PDU |
|-------------|------------------|---------------|
| Subrecords longueur 0 (HDR seul) | jusqu’à **4095** | 0 |
| Remplissage optimal (ci-dessus) | **64** | **4031** |

Le **crédit** ([[SSID]], `SSIDCRED`) limite le nombre de **PDU DATA** consécutifs, pas le nombre de subrecords **dans** une PDU.

## Flow control

- `V.Credit_S` décrémenté à chaque envoi (Action 13, 9.10).
- Si `Credit_S - 1 = 0` → état **OPOWFC**, attendre [[CDT]] (envoyé par le **Listener**, pas par le Speaker).
- Listener : quand `Credit_L` bas, envoyer **CDT** (9.12 **I**, P7).

## Transitions

| Côté | Déclencheur | Code | Résultat |
|------|-------------|------|----------|
| Speaker | `F_DATA_RQ` | M | DATA → OPO ou OPOWFC |
| Listener | DATA | I | `F_DATA_IND` (+ CDT si besoin) |

---

## Exemple RFC — Annexe A (*The Rime of the Ancient Mariner*)

Source : RFC 5024 **Appendix A** (fichier format **T**, lignes séparées par **CR-LF**). Coleridge, *The Rime of the Ancient Mariner*.

### Fichier virtuel (extrait)

```text
             It is an ancient Mariner,
             And he stoppeth one of three.
             "By thy long grey beard and glittering eye,
             Now wherefore stopp'st thou me?

             "The Bridegroom's doors are opened wide,
             And I am next of kin;
             The guests are met, the feast is set:
             May'st hear the merry din."

             He holds him with his skinny hand,
             "There was a ship," quoth he.
             "Hold off! unhand me, grey-beard loon!"
             Eftsoons his hand dropt he.

             He holds him with his glittering eye--
             The Wedding-Guest stood still,
             And listens like a three years; child:
             The Mariner hath his will.

             The Wedding-Guest sat on a stone:
             He cannot chuse but hear;
             And thus spake on that ancient man,
             The bright-eyed Mariner.

             The ship was cheered, the harbour cleared,
             Merrily did we drop
             Below the kirk, below the hill,
             Below the light-house top.
```

### Légende de la RFC (vue ASCII des buffers)

| Symbole | Signification |
|---------|----------------|
| `D` | Commande DATA (`DATACMD`) |
| `?` | Octet **HDR** du subrecord (la ligne hex en dessous donne la vraie valeur, ex. `0x43`, `0x74`…) |
| `..` | Dans l’extrait RFC : séparateur visuel entre **enregistrements** (CR-LF du format T), pas un octet littéral `'.'` |

Chaque subrecord = **`?` + texte** jusqu’au prochain `?`. Les buffers peuvent être **plus courts** que `Buf-size` (§7.3).

### Exchange Buffer 1 (transcription RFC)

Ligne 1 = caractères ; lignes suivantes = hex (RFC).

```text
D?It is an ancient Mariner,..And he stoppeth one of three..."By
 t?hy long grey beard and glittering eye,..Now wherefore stopp'st
  ?thou me?...."The Bridegroom's doors are opened wide,..And I am
  ?next of kin;..The guests are met, the feast is set:..May'st he
 a?r the merry din."....He holds him with his skinny hand,.."Ther
 e? was a ship," quoth he..."Hold off! unhand me, grey-beard loon
 !?"..Eftsoons his hand dropt he.....He holds him with his glitte
 r?ing eye--..The Wedding-Guest stood still,..And listens like a
 t?hree years; child:..The Mariner hath his will.....The Wedding-
 G?uest sat on a stone:..He cannot chuse but hear;..And thus spak
 e? on that ancient man,..The bright-eyed Mariner.....The ship wa
 s? cheered, the harbour cleared,..Merrily did we drop..Below the
  .kirk, below the hill,..Below the light-house top...
```

Hex associé (début buffer 1, RFC) :

```text
4347267266266666672467666720046626627767767626662662767662002472
4F9409301E01E395E40D129E52CDA1E4085034F005480FE50F6048255EDA2290
…
```

*(Le fichier complet de l’annexe ne tient que dans **Exchange Buffer 1** dans cet exemple ; la RFC enchaîne hex sur plusieurs lignes par buffer.)*

### Ce que l’exemple illustre

- Plusieurs **subrecords** par PDU : chaque `?` = nouveau HDR + suite de texte (souvent ≤ 63 o de données par morceau).
- Un seul **`D`** par exchange buffer.
- Fichier **texte structuré** (format T) : les coupures suivent le contenu et les CR-LF, pas seulement `Buf-size`.
- En implémentation : parser `DATABUF` en alternance **HDR → payload de longueur COUNT**, pas un bloc brut `Buf-size - 1`.

Référence complète : `RFCs/rfc5024.txt` §Appendix A (vers lignes 5551+).

Échange **bout en bout** (session + `hello.txt` / « HELLO WORLD », chaque PDU détaillée) : [[../../02-cas-speciaux/exemple-echange-hello-world]].

---

## OFTP1 vs OFTP2

| Aspect | OFTP1 | OFTP2 |
|--------|-------|-------|
| `DATACMD` | `'D'` | `'D'` |
| `DATABUF` | **String(n)** | **Binary(n)** — données binaires après chiffrement/compression |

Sémantique identique : un buffer DATA par exchange buffer, flow control CDT inchangé.

## Implémentation

- [ ] `V.Buf-size` = min(local, pair) après SSID
- [ ] Encoder §7 : HDR (EOR/CF/COUNT) ; CF=0 → payload ≤ 63 o ; CF=1 → 1 octet + COUNT répétitions
- [ ] Si `V.Compression` : détecter runs ; sinon toujours CF=0
- [ ] Décodeur : branche CF=0 (lire COUNT octets) vs CF=1 (répéter 1 octet × COUNT)
- [ ] Vérifier `1 + len(DATABUF) ≤ V.Buf-size` avant envoi
- [ ] Sur TCP : STB (STH + OEB) **en plus** du DEB (§8)
- [ ] Ne pas envoyer si `OpOutWaitCredit` ; attendre [[CDT]]
- [ ] Tests : reprendre l’annexe A (Mariner) ou comparer hex RFC
