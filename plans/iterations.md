# Tanulokartyas Rust GUI Alkalmazas - Iteracios Terv

## Projekt cel
Egy modern kinezetu, asztali Rust GUI alkalmazas fejlesztese tanulokartyakhoz, ahol a kartyak es beallitasok JSON fajlban tarolodnak.

## Javasolt technologiai irany
- GUI: Dioxus Desktop (modern UI komponensek + CSS-szeru stiluskezeles)
- Szerializacio: serde + serde_json
- Hiba kezeles: anyhow / thiserror
- Datum es idobelyeg: chrono
- Teszteles: unit + integracios tesztek a domain es persistence retegre

## Kotott technikai kovetelmenyek (tanari elvaras)
- A kepen jelzett Rust technikak kozul minimum 2 kotelezoen alkalmazando a projektben.
- Kotelezoen hasznalt elemek (javasolt es a tervben rogzitett):
  - TryFrom / TryInto (adatkonverziohoz es validaciohoz)
  - Arc (megosztott allapot vagy szalbiztos referencia eseten)
  - macro_rules! (ismetlodik kodmintak kivaltasara)
- A minimum 2 kovetelmeny teljesulesehez eleg 2 technika is, de a cel legalabb 3 tudatos alkalmazasa.
- Ahol csak lehet, makrokat hasznalunk: elsodlegesen sajat macro_rules! makrokat, masodlagosan szabvanyos derive/attribute makrokat.
- Minden hasznalt technikahoz rovid indoklas kerul a dokumentacioba (hol es miert lett alkalmazva).

Megjegyzes: Ha az oktato kifejezetten mas frameworkot ker (pl. Slint), a terv szerkezete valtozatlanul alkalmazhato.

---

## Iteracio 0 - Alap alkalmazas felepitese (KOTELEZO kezdolpes)
### Cel
Mukodokepes minimum projektvaz letrehozasa tiszta retegezessel, hogy erre lehessen biztonsagosan epiteni.

### Feladatok
1. Projekt bootstrap
- Cargo projekt letrehozasa GUI template-tel.
- Modulstruktura kialakitasa:
  - src/app (UI allapot + routing)
  - src/domain (Flashcard, Deck, Session modellek)
  - src/services (json storage, validacio)
  - src/ui (kepernyok, komponensek, theme)

2. Domain modellek elokeszitese
- Alap entitasok:
  - Flashcard: id, front, back, tags, created_at, updated_at
  - Deck: id, name, description, cards
  - AppState: deckek, aktiv deck, beallitasok
- Serde derive-ok elokeszitese (Serialize/Deserialize).
- Konverzios reteg alapja TryFrom/TryInto mintaval (pl. input -> domain tipus).

3. Alap UI shell
- Foablak + bal oldali navigacio + fo tartalom terulet.
- Kezdo dashboard ures allapot kezelessel.
- Alap komponensek: gomb, input, kartya panel, modal vaz.

4. Theme alapok (modern kinezet alap)
- Design tokenek bevezetese (szinek, spacing, radius, shadow, typo).
- Egyseges stilus: vilagos alaptema, eros kontraszt, letisztult tipografia.
- Reszponziv torontesek desktop meretekre (small/medium/large window).

5. JSON storage skeleton
- data/app_state.json path konvencio.
- Elso inditasnal default allapot letrehozasa.
- Betoltes/mentes API alairasok:
  - load_app_state() -> Result<AppState>
  - save_app_state(&AppState) -> Result<()>

6. Makro strategia inditasa
- 1-2 sajat macro_rules! makro tervezese az ismetlodo mintakra (pl. validacios hiba-epites, UI helper boilerplate).
- Makro hasznalati szabaly: csak ott, ahol valoban csokkenti az ismetlest es javitja az olvashatosagot.

### Kimenet
- Fordithato, indithato GUI alkalmazas skeleton.
- Mukodo JSON beolvasas/mentes minimal allapottal.
- Dokumentalt modulhatarok.

### Elfogadasi kriteriumok
- Az app elindul crash nelkul.
- Letrejon vagy beolvasodik a JSON allapotfajl.
- Minimum 1 ures dashboard kepernyo es navigacio lathato.
- Legalabb 1 sajat macro_rules! makro tenylegesen be van kotve a kodba.

---

## Iteracio 1 - Alap tanulokartya CRUD
### Cel
A felhasznalo tudjon deckeket es kartyakat letrehozni, szerkeszteni, torolni.

### Feladatok
1. Deck kezeles
- Deck lista nezet (kereso + rendezes nev szerint).
- Uj deck modal validacioval.
- Deck szerkesztes es torles megerositessel.

2. Flashcard CRUD
- Kartya lista nezet decken belul.
- Uj kartya felvetele (front/back kotelezo mezok).
- Inline vagy modal szerkesztes.
- Torles megerosito parbeszedablakkal.

3. JSON perzisztencia bekotese
- Minden CRUD muvelet utan autosave.
- Tranzakcios mentesi szemlelet: temp fajl + atomikus csere.

4. Technikak explicit hasznalata
- TryFrom/TryInto alkalmazasa bemeneti modellek biztonsagos atalakitasara.
- Macro hasznalat a validacios/hibakezelesi boilerplate csokkentesere.

### Kimenet
- Teljes alap CRUD folyamat deckekre es kartyakra.

### Elfogadasi kriteriumok
- App ujrainditas utan minden modositas megmarad JSON-ban.
- Validacio hiba esetben ertheto hibauezenet jelenik meg.
- A valasztott technikak kozul legalabb ketto mar bizonyithatoan jelen van a kodban.

---

## Iteracio 2 - Tanulasi mod (flip + ertekeles)
### Cel
Mukodo tanulasi folyamat kialakitasa, ahol a kartya megfordithato es ertekelheto.

### Feladatok
1. Session logika
- Tanulasi session inditasa deckbol.
- Kartya sorrend: shuffle opcio.
- Allapot: aktualis index, mutatott oldal (front/back), pontozas.

2. Interakciok
- Flip gomb / billentyuparancs.
- Ertekeles: Nehez / Kozepes / Konnyu.
- Kovetkezo kartya navigacio.

2.1 Halado Rust minta beemelese
- Arc hasznalata ott, ahol a GUI allapotot vagy szolgaltatast megosztottan kell atadni.
- Closure/HOF mintak alkalmazasa callbackeknel es listafeldolgozasnal.

3. Session osszegzes
- Session vegi statisztika: osszes kartya, ido, ertekeles megoszlas.
- Eredmeny mentes JSON-ba (history tomb).

### Kimenet
- Vegigviheto tanulasi session.

### Elfogadasi kriteriumok
- A session allapot nem torik meg gyors kattintasoknal sem.
- Session vegi adatok visszakereshetok.

---

## Iteracio 3 - Modern UI finomitas es UX erosites
### Cel
A vizualis minoseg es hasznalhatosag felhozatala modern desktop app szintre.

### Feladatok
1. Vizualis rendszer
- Váltás világos és sötét téma között a beállításokban.
- A Dashboard résznél töltsd fel kártyákkal, hogy ne csak 1 kártya legyen, töltsd ki a teret, akár lehetne középre igazított hero section féleség is, ami kicsit leírja, hogy mégis mi ez az alkalmazás.
- Kiforrott komponensstilusok: gombvariansok, input allapotok, kartya animaciok.
- Tipografiai hierarchia (display, heading, body, caption).
- Tudatos szinpaletta (primer, semleges, siker/figyelmeztetes/hiba).
- Állapot mentése gomb lehetne csak egy floppy ikon, ha ráviszem az egeret, akkor tooltipbe írja, hogy "Állapot mentése".
- A "Deck" szót mindenhol cseréljük le Paklira, a magyarosabb érzet kedvéért.
- Az "Iteracio 2 tanulas mod" meg az "Iteracio 1-2: CRUD + tanulasi session inditas" szövegek helyett valami általánosabb leírás legyen az adott dologhoz.

2. Mozgas es visszajelzes
- Finom animaciok: kartya flip, modal nyitas, lista elemek beluszasa.
- Loading, empty state, success/error toast.

3. UX fejlesztes
- Billentyuparancsok (pl. Space flip, nyilak navigacio).
- Fokuszmenedzsment es alap akadalymentesites (tab sorrend, aria-jellegu cimkezes ahol tamogatott).

### Kimenet
- Egyseges, modern es kenyelmesen hasznalhato UI.

### Elfogadasi kriteriumok
- Kepernyok kozotti valtas vizualisan konzisztens.
- Alap felhasznaloi utak (CRUD + tanulas) eger nelkul is vegigvihetok.

---

## Iteracio 4 - Stabilitas, hibakezeles, tesztek
### Cel
Megbizhato alkalmazas release-kozeli minosegben.

### Feladatok
1. Hibakezeles
- Kozponti error tipusok.
- Felhasznalobarat hibauezenetek I/O es parse hibakra.
- Sertetlen allapot garancia hibas JSON esetben (backup/repair flow).

2. Teszteles
- Domain unit tesztek (validacio, session logika).
- Persistence integracios tesztek (load/save, migracio).

3. Minosegbiztositas
- clippy + fmt + teszt pipeline.
- Alap teljesitmenyellenorzes nagyobb kartyamennyiseggel.

### Kimenet
- Dokumentalt, tesztelt stabil alkalmazas.

### Elfogadasi kriteriumok
- Kritikus use-case-ek automata tesztekkel lefedve.
- Hibas input JSON eseten sem vesznek el adatok.

---

## Iteracio 5 - Csomagolas es beadando veglegesites
### Cel
Atadhato, bemutathato projekt keszitese.

### Feladatok
1. Build es release
- Windows target build.
- Futathato csomag es telepithetoseg (ha kovetelmeny).

2. Dokumentacio
- README: futtatas, funkciok, projekt struktura, ismert korlatok.
- Kepernyokep gyujtemeny (modern UI bemutatas).

3. Demo adatok
- Pelda deck + 15-20 kartya JSON-ban.
- Gyors bemutato workflow leiras.

### Kimenet
- Beadhato projektcsomag es demo.

### Elfogadasi kriteriumok
- Uj gepen, friss klon utan reprodukalhato build/futtatas.
- A modern kinezet es JSON mentes egyertelmuen igazolhato.

---

## Munkaszervezesi ritmus (javaslat)
- Iteracio hossza: 3-5 nap / iteracio.
- Minden iteracio vegi checkpoint:
  - Mukodo build
  - Frissitett dokumentacio
  - Demo video vagy rovid bemutato jegyzet

## Kockazatok es kezelesuk
1. GUI framework tanulasi gorbe
- Kezeles: Iteracio 0-ban minimal PoC kepernyo nagyon hamar.

2. JSON adatvesztes hibas mentesnel
- Kezeles: atomikus mentes + backup fajl.

3. UI modernitas szubjektiv
- Kezeles: elore definialt design tokenek es referencia kepernyok Iteracio 0-1 kozott.

## Definicio of Done (teljes projekt)
- Deck + kartya CRUD stabilan mukodik.
- Tanulasi session flip + ertekelessel mukodik.
- Minden adat JSON-ban perzisztalodik ujrainditas utan is.
- UI vizualisan konzisztens, modern es jol hasznalhato.
- Van dokumentacio es futtathato release build.
- Minimum 2, a listabol valasztott Rust technika hasznalata kodszinten egyertelmuen igazolt.
- Ahol ertelmes, makro (kulonosen macro_rules!) hasznalat valositja meg az ismetlodo mintakat.
