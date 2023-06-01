# hourlistparser

Parses hourlists to the number of hours for each project

Jeg skriver timelistene i denne formen

```
- 09:00 Prosjekt a
- 10:00 Prosjekt b
- 11:30 Prosjekt a
- 12:00 Lunsj
- 12:30 Prosjekt b
- 17:00 Ferdig
```

Dette cli-et kalkulerer antall timer for hvert prosjekt

```
cat | cargo run
```

og så limer du inn timelistene
