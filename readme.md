# Naplánování vypnutí (Scheduled shutdown)

Jednoduchá konzolová aplikace pro naplánování vypnutí počítače po určeném čase.

## Funkce

- **Plánované vypnutí:** Nastavení vypnutí počítače za zadaný počet minut
- **Zrušení vypnutí:** Možnost zrušit již naplánované vypnutí
- **Podpora více OS:** Kompatibilita s Windows i Linux systémy
- **Zpětná vazba:** Informativní hlášky o stavu naplánovaného vypnutí
- **Jednoduchý interface:** Intuitivní ovládání pomocí jednoduchých příkazů

## Použití

Po spuštění aplikace se zobrazí prompt `>_`, kde můžete zadat příkazy:

1. **Naplánování vypnutí:**

   - Zadejte číslo (počet minut)
   - Příklad: `>_ 30` - vypnutí za 30 minut

2. **Zrušení naplánovaného vypnutí:**
   - Zadejte příkaz `stop`
   - Příklad: `>_ stop`

## Chybové stavy

- Pokud zadáte příkaz `stop` bez předchozího naplánování vypnutí, aplikace vás upozorní
- Pokud zadáte nečíselnou hodnotu, aplikace vás informuje o neplatném vstupu
- Pokud již máte naplánované vypnutí a pokusíte se nastavit nové, aplikace vás upozorní, že nejprve musíte zrušit předchozí plán

## Technické detaily

- Aplikace je napsaná v Rustu
- Využívá vícevláknové zpracování pro odpočítávání času
- Zajišťuje korektní vypnutí počítače pomocí systémových příkazů

## Instalace

1. Zkompilujte projekt pomocí příkazu:

   ```
   cargo build --release
   ```

2. Spusťte aplikaci:
   ```
   cargo run
   ```
   nebo přímo spustitelný soubor ve složce `target/release`

## Systémové požadavky

- Windows nebo Linux operační systém
- Nainstalovaný Rust a Cargo pro kompilaci
