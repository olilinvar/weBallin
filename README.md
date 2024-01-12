# Årsoppgave Våren 2024

Dette er generell info om Oliver og Trygve sin årsoppgave

### Notat til lærer

```
Dette er en sammarbeidsoppgave
```

```
Vi ønsker å gjøre det velding enkelt for dere å se hvem som har jobbet med hva slik at dere kan gi en individell vurdering. Dette kommer med github om man velger et repo, trykker på "Insight" kommer man til en side der man for oversikt over hvor mange linjer kode forskjellige personer har skrevet.
```

## Hva

Vi skal lage et 2D hack'n'slash plattformspill fra bunnen av, uten bruk av en eksisterende game engine. Vi skal implementere grafikk og vindushåndtering ved hjelp av bibliotekene OpenGL og GLFW.

I tillegg til egenlaget kode vil vi være ansvarlige for det grafiske designet av alle visuelle elementer, inkludert animasjoner, sprites, karakterdesign og bakgrunnsdesign. Vi skal også skape lydeffekter og musikk for spillet.

Dette skal komme med en nettside for å laste ned spillet.

## Hvordan

Vi kommer til å følge en grundig planleggingsprosess ved å bruke et Github-prosjekt for å holde oversikt over fremdriften vår. Planen, veikartet og oppgavelisten vil være tilgjengelige på Github for en enkel og strukturert oversikt over prosjektet. I tillegg til Github vil vi ha minimum ett møte i uken for å diskutere fremdrift og eventuelle utfordringer.

Vi planlegger å bruke eksisterende biblioteker for de mest komplekse delene av koden, som vindushåndtering, inputhåndtering og grafikk. Disse bibliotekene vil bli implementert som en del av den større løsningen.

For å sikre et et godt resultat vil vi gjennomføre dokumenterte brukertester senere i løpet.

Oppgaven stiller med mange krav, for å nå disse skal vi ha funksjonaliteter som lagring og et ledertavlesystem som sporer tid brukt på hvert nivå. I tilleg skal vi ha en nettside hvor man kan laste ned en exe fil av spillet og da logge inn på siden for å så se din egen poengsum og tid (usikkert).

Denne nettsiden skal kjøre på en podman container med Nginx og en Database.

For å beskytte vårt håndsverk, men fortsatt gjøre det tilgjengelig å modifisere og endre for en selv kommer vi til å legge verke vårt under et lisens.

## Hvorfor

Vi ønsker å utvikle et engasjerende 2D spill for å utvikle våre ferdigheter innen C++-programmering, spillutvikling og grafisk design. Samtidig som vi lærer om teamarbeid, organisering og planlegging.

Utenom det vil vi også lære om conteinere.

## Redskaper/Programvare/Biblioteker

- **Operativsystem:**
  - Windows 10/11
  - Debian (Linux)
  - Nginx (container)

- **Programvare:**
  - Krita og Aseprite for grafisk design
  - bfxr og LMMS for lydeffekter og mussik
  - Podman for håndtering av containere

- **Biblioteker:**
  - OpenGL for grafikk
  - GLFW for vindushåndtering

### Databasesystem
Vi ønsker å bruke MongoDB som databasesystem for lagring av spilldata

### TODO (Forenklet)
## Spill
- [x] Lage prosjektbeskrivelse
- [ ] Konfigurering nødvendige verktøy
- [ ] Fikse vindushåndtering
- [ ] Integrer OpenGL
- [ ] Legg til spillomerådet og karakter
- [ ] Implementer Bevegelseskontroller
- [ ] Grafisk design
- [ ] Spillmekanikker
- [ ] Lyddesign
- [ ] Databaseintegrering
- [ ] Brukerteste
- [ ] Finpusse og Finalisering

## Nettside
- [ ] Sette opp en Debian VM
- [ ] Sette opp Podman på Debian VM
- [ ] Konfigurere Nginx container 
  - [ ] Konfigurere database
- [ ] Lage nettsiden

### Annet (Veldig forenklet)
- [ ] Nettside med download
- [ ] FAQ


## Utviklet av

Dette var utviklet av:
[Oliver](https://github.com/olilinvar)
[Trygve](https://github.com/TheCyberiousPizzerious)
