# Tanulokartya GUI - Iteracio 1

Ez a repository az Iteracio 1 kovetelmenyeit teljesito, fordithato desktop GUI alkalmazas.

## Technologia
- Rust + eframe/egui desktop GUI
- serde + serde_json JSON perzisztencia
- anyhow + thiserror hibakezeles
- chrono idobelyegek

## Modulhatarok
- `src/app`: UI allapot, routing, app orchestration
- `src/domain`: alap entitasok, input modellek, TryFrom konverzio
- `src/services`: validacio es JSON storage
- `src/ui`: komponensek, kepernyok, tema

## Iteracio 1 teljesules
- Deck CRUD: letrehozas, szerkesztes, torles megerositessel
- Deck lista: kereses + nev szerinti rendezes
- Flashcard CRUD a kijelolt decken belul: letrehozas, szerkesztes, torles megerositessel
- Minden CRUD muvelet utan automatikus JSON mentes
- Tranzakcios mentes: temp fajl + atomikus csere
- `TryFrom/TryInto` hasznalat input es update modellek validalt konverziojahoz
- Sajat `macro_rules!` makro hasznalat validacios hibakhoz: `validation_error!`
- `Arc<JsonStorage>` hasznalat a megosztott storage referenciara

## Futtatas
```bash
cargo run
```
