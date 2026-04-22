# Tanulokartya GUI - Iteracio 0

Ez a repository az Iteracio 0 kovetelmenyeit teljesito, fordithato desktop GUI skeleton.

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

## Iteracio 0 teljesules
- GUI shell bal oldali navigacioval es dashboarddal
- `data/app_state.json` automatikus letrehozas elso inditasnal
- `load_app_state() -> Result<AppState>` es `save_app_state(&AppState) -> Result<()>`
- legalabb egy sajat `macro_rules!` makro: `validation_error!`
- `TryFrom` alap deck/kartya input konverzio
- `Arc<JsonStorage>` hasznalat a megosztott storage referenciara

## Futtatas
```bash
cargo run
```
