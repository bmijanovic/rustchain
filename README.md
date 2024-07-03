# Specifikacija Projekta: Implementacija Blockchaina sa Kriptovalutom i Pametnim Ugovorima u Rustu

## 1. Uvod

Cilj ovog projekta je izrada blockchain sistema koji podržava kriptovalutu i pametne ugovore koristeći programski jezik Rust. Sistem će obuhvatiti sve ključne komponente potrebne za funkcionalan i siguran blockchain, uključujući proof-of-work (PoW) konsenzusni mehanizam, digitalni wallet za korisnike, mrežni sloj za razmenu transakcija, kao i mogućnost kreiranja i izvršavanja pametnih ugovora.

## 2. Ciljevi Projekta

- Implementirati osnovni blockchain sa PoW mehanizmom.
- Razviti digitalni wallet za korisnike.
- Implementirati mrežni sloj za razmenu i validaciju transakcija.
- Osigurati sigurnost i performanse sistema.

## 3. Tehnologije

- **Programski jezik:** Rust
- **Biblioteke:** Serde (za serijalizaciju i deserijalizaciju), Tokio (za asinhrono programiranje), Ring (za kriptografske funkcije)
- **Alati:** Wasm-pack, Cargo

## 4. Funkcionalnosti

### 4.1 Blockchain

- **Struktura bloka:** Svaki blok sadrži heder (hash prethodnog bloka, timestamp, nonce) i listu transakcija.
- **Dodavanje blokova:** Funkcija za dodavanje novih blokova uz proveru validnosti putem PoW mehanizma.
- **Validacija lanca:** Funkcija za validaciju celog lanca blokova kako bi se osigurao integritet i konzistentnost.

### 4.2 Proof-of-Work

- **Hash funkcija:** Korišćenje SHA-256 za kreiranje hash vrednosti.
- **Rudarenje:** Algoritam za minovanje koji traži validan nonce kroz iterativno hashovanje dok se ne pronađe odgovarajući hash koji zadovoljava uslove težine.
- **Podesiva težina:** Mehanizam za prilagođavanje težine PoW u zavisnosti od brzine generisanja blokova.

### 4.3 Transakcije

- **Struktura transakcije:** Transakcije uključuju pošiljaoca, primaoca, iznos i digitalni potpis.
- **Verifikacija transakcija:** Provera valjanosti transakcije kroz digitalne potpise i dostupnost sredstava.

### 4.4 Wallet

- **Generisanje ključeva:** Generisanje parova javnog i privatnog ključa koristeći kriptografske funkcije.
- **Potpisivanje transakcija:** Korišćenje privatnog ključa za potpisivanje transakcija.
- **Slanje i primanje sredstava:** Funkcionalnosti za upravljanje sredstvima korisnika.

### 4.5 Mrežni sloj

- **P2P mreža:** Implementacija peer-to-peer mreže za razmenu blokova i transakcija.
- **Sinhronizacija lanca:** Mehanizmi za sinhronizaciju lanca blokova među čvorovima mreže.
- **Broadcast transakcija:** Slanje transakcija svim čvorovima u mreži radi validacije i uključivanja u nove blokove.
