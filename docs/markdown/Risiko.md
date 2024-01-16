# Risiko

I dette dokumentet gjennomgås det risikovurderinger og risikotiltakene for nevdte vurderinger

## Risikovurdering

!VI MÅ LAGE ET RISIKODIAGRAM OG MERE RISIKOER (kommer nok naturlig jo mer vi holder på)

I prosjektet er det ikke mange risikoer siden det er ment til å være open source. At det er open source betyr at det skal kunne være mulig å se, laste ned og endre koden. Siden koden er åpen er det ikke farlig om noen finner ut om koden.

Det er ikke helt risikofritt, følgene er alle sikkerhets risikoer (så langt):

|Applikasjon     |Tiltak (utdypt) |
|----------------|----------------|
|Netverk         |t1              |
|Database        |t2              |
|Naturkatastrofer|t3              |
|Datatap         |t4              |

## Risikotiltak

### t(iltak)1

Risikoen er om noen uønskede får tilgang til nettverket og aplikasjonene der. Nettsiden skal kjøre i en container så mye utfordringer blir løst. Det er fortsatt noen utfordringer som om noen får tilgang til containeren på et vis. Containere er med mindre endret "rootless" altså de har ikke root, så det er ikke noe root passord eller root bruker. I linux kan du legge til superbrukere, men dette krever en superbruker fra før av. Så ingen superbruker ingen tilgang. Alt du trenger blir lagt til før containeren lages.

### t(iltak)2

Vi har en database som lagrer brukerens tid på forskjellige nivåer. Brukeren skal ikke logge inn med email, tanken er at brukere skal logge inn med brukernavn og passord. Det er ikke spesielt farlig om man lager nye brukere eller så kan vi ha en fil på brukeren sin datamaskin som identifiserer dem om det skulle vise seg å være et problem. Brukernavnene skal lagres i en database denne databasen skal kjøre på samme container som nettsiden. Dette er av flere grunner, 1) Det er simplere å kjøre det i samme container. 2) Kjøre i forskjellige containere åpner nye sikkerhetsproblemer. Ikke gå over bekken etter vann, ikke gjør det mer komplisert enn det trenger å være. Om database og nettside kjører i forskjellige containere må databasen hostes på nettet for at containeren som kjører nettsiden skal finne den. Dette gjør slik at alle kan koble seg til databasen også uønskede.

### t(iltak)3

Dette er nok den 