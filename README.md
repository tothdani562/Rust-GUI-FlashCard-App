# Tanulokartya GUI - Iteracio 5

Egy modern, asztali Rust GUI alkalmazas tanulokartyakhoz. A projekt veglegesitett, bemutathato allapotban van: futtathato desktop app, JSON perzisztencia, demo adat, release build, es a beadashoz szukseges rovid dokumentacio.

## Mit tud az alkalmazas?
- Pakli- es kartyakezeles.
- Tanulasi mod fordithato kartyakkal es ertekelessel.
- Automatikus JSON mentes/olvasas a `data/app_state.json` fajlbol.
- Modern, egerrel es billentyuzettel is jol hasznalhato felulet.

## Futtatas
Fejlesztoi inditas:
```bash
cargo run
```

Release build Windowsra:
```bash
cargo build --release
```

A kesz futtathato allomany a `target/release/app_gui.exe` utvonalon jon letre.

## Gyors bemutato workflow
A reszletes leiras a [docs/demo-workflow.md](docs/demo-workflow.md) fajlban van. A rovid verzio:
1. Inditsd el az appot.
2. Valaszd a `Rust alapok` paklit.
3. Indits tanulasi sessiont.
4. Fordits `Space`-szel, lepj tovabb a nyilakkal vagy a kovetkezo gombbal, ertekelj `1`, `2`, `3` billentyuvel.
5. Menteshez hasznald a floppys ikont.

## Projektstruktura
- `src/app`: UI allapot, routing, app orchestration
- `src/domain`: entitasok, input modellek, TryFrom konverziok
- `src/services`: validacio es JSON storage
- `src/ui`: komponensek, kepernyok, tema
- `data/app_state.json`: alap demo allapot es perzisztalt adat

## Demo adat
A `data/app_state.json` jelenleg egy bemutathato, 16 kartyas `Rust alapok` paklit tartalmaz. Ez elegendo arra, hogy a dashboard, a pakli lista, a tanulasi mod es a JSON perzisztencia azonnal demonstralhato legyen friss klon utan is.

## Kepernyokep gyujtemeny
A projekt jelenlegi allapotahoz a kovetkezo kepernyokep-csomag ajanlott a beadashoz:
- Dashboard / kezdokepernyo
- Pakli lista es szerkesztes
- Tanulasi mod elolap es hatoldal
- Ertekelesi allapot es kovetkezo kartyara lepes
- Beallitasok vilagos es sotet tema valtassal

Ha a vegso beadashoz kulon kepeket szeretnel tarolni, egy `docs/screenshots/` mappa jol illeszkedik a projekt strukturajahoz.

## Ismert korlatok
- Nincs kulon installer vagy telepito csomag.
- A perzisztencia egy lokalis JSON fajlon alapul.
- A screenshot gyujtemenyhez jelenleg nincs repo szintu kepelem, csak a javasolt keplista.

## Modulhatarok
- `src/app`: UI allapot, routing, app orchestration
- `src/domain`: alap entitasok, input modellek, TryFrom konverzio
- `src/services`: validacio es JSON storage
- `src/ui`: komponensek, kepernyok, tema
