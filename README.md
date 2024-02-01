# Årsoppgave Våren 2024

Dette er generell info om Olivers og Trygves årsoppgave.

### Notat til lærer

Dette er en samarbeidsoppgave.

Vi ønsker å gjøre det veldig enkelt for dere å se hvem som har jobbet med hva, slik at dere kan gi en individuell vurdering. Dette kommer med GitHub. Hvis man velger et repo, trykker på "Insight", kommer man til en side der man får oversikt over hvor mange linjer kode forskjellige personer har skrevet.

## Hva

Nettisde som er fiktivt relatert til et spill. På denne nettsiden skal man kunne logge inn for å se og redigere brukerprofil, hvor brukeren da er linket til noen highscores som også skal kunne slettes. Det skal ha en REST API backend som er skrevet i Rust og frontenden er skrevet i react. Om vi har tid til overs skal vi begynne utvikling på nevdt spill.

## Hvordan

Vi kommer til å følge en grundig planleggingsprosess ved å bruke et GitHub-prosjekt for å holde oversikt over fremdriften vår. Planen, veikartet og oppgavelisten vil være tilgjengelige på GitHub for en enkel og strukturert oversikt over prosjektet. I tillegg til GitHub vil vi ha minimum ett møte i uken for å diskutere fremdrift og eventuelle utfordringer.

Vi planlegger å bruke eksisterende biblioteker for de mest komplekse delene av koden, som vindushåndtering, inputhåndtering og grafikk. Disse bibliotekene vil bli implementert som en del av den større løsningen.

For å sikre et godt resultat vil vi gjennomføre dokumenterte brukertester senere i løpet.

Oppgaven stiller med mange krav, for å nå disse skal vi ha funksjonaliteter som lagring og et ledertavlesystem som sporer tid brukt på hvert nivå. I tilleg skal vi ha en nettside hvor man kan laste ned en exe fil av spillet og da logge inn på siden for å så se din egen poengsum og tid (usikkert). Denne nettsiden skal kjøre på en podman container med Nginx og en Database.

For å beskytte vårt håndverk, men fortsatt gjøre det tilgjengelig for å modifisere og endre for en selv, kommer vi til å legge verket vårt under et lisens.

## Hvorfor

Vi ønsker å utvikle et engasjerende 2D spill for å utvikle våre ferdigheter innen C++-programmering, spillutvikling og grafisk design. Samtidig som vi lærer om teamarbeid, organisering og planlegging.

Utenom det vil vi også lære om containere.

## Redskaper/Programvare/Biblioteker

<!-- a b c d e f g h i j k l m n o p q r s t u v w x y z -->

- **Operativsystem:**
  - Nginx (container) - Hosting av nettside og database
  - Windows 10/11 - Utviklingsplattform

- **Programvare:**
  - Aseprite - Grafisk design
  - bfxr - Lydeffekter
  - CMake - Bygging og kompilering
  - Krita - Grafisk design
  - LMMS - Lydeffekter og musikk
  - Podman - Containere
  - Visual Studio Code - IDE

- **Biblioteker:**
  - OpenGL - Grafikk
  - SFML - Håndtering av vinduer, lyd og input

- **Databasesystem**
  - MongoDB - Database

<!--Dette trenger vi ikke gjøre veldig detaljert, men kan utvides-->

### TODO (Forenklet)
## Spill
- [x] Lage prosjektbeskrivelse
- [ ] Konfigurere nødvendige verktøy
- [ ] Fikse vindushåndtering
- [ ] Integrere OpenGL
- [ ] Legge til spillområdet og karakter
- [ ] Implementere bevegelseskontroller
- [ ] Grafisk design
- [ ] Spillmekanikker
- [ ] Lyddesign
- [ ] Databaseintegrering
- [ ] Brukertesting
- [ ] Finpusse og finalisering

## Nettside
- [ ] Sette opp en Debian VM
- [ ] Sette opp Podman på Debian VM
- [ ] Konfigurere Nginx-container 
  - [ ] Konfigurere database
- [ ] Lage nettsiden
  - [ ] FAQ-side

## Utviklet av

[Oliver](https://github.com/olilinvar)
[Trygve](https://github.com/TheCyberiousPizzerious)
