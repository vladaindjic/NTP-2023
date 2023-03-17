

# Instalacija Pharo okruženja

Da biste koristili *Pharo* okruženje, neophodno je da instalirate i pokrenete Pharo virtualnu mašinu.
Najjednostavniji način da ovo uradite jeste preko [*Pharo Launcher*-a](https://pharo.org/download).
Njega ćete instalirati prateći [sledeće uputstvo](https://pharo-project.github.io/pharo-launcher/installation/) u zavisnosti od operativnog sistema koji koristite.
Nakon uspešne instalacije, potrebno je pokrenuti pomenuti *Launcher*. Zatim, treba kreirati virtualnu
mašinu po uzoru na [sledeće uputstvo](https://pharo-project.github.io/pharo-launcher/create-images/). Ukratko, izbog opcije *New*, zatim izbor stabilne verzije mašine i kreiranje slike.
Sačekati neko vreme da se mašina povuče sa repozitorijuma i izgradi. Nakon toga, potrebno je pokrenuti mašinu.
To radite selektovanjem virtualne mašine i izborom opcija *Launch* (*play* dugme). Detaljnije upustvo možete
pronaći [ovde](https://pharo-project.github.io/pharo-launcher/launch-configurations/).

Ukoliko na vežbama koristite fakultetske računare, tada ćete najverovatnije morati da podesite *proxy*.
To radite izborom opcije *Settings* (gornji desni ugao launchera). Zatim otvarate *Network* listu
i čekirate *Use HTTP proxy...* dugme. U zavisnosti od učionica, za server postavljate:
- `192.168.77.100` (za učionice Računarskog centra)
- `proxy.uns.ac.rs` (za učionice u NTP-u).
Port bi u oba slučaja trebalo da bude `8080`. Prilikom unosa serva i porta, morate pritistnuti
taster *Enter* kako bi launcher sačuvao Vaš unos.

**Napomena 1**: Ukoliko donosite svoje računare, dobro bi bilo da instalirate *Launcher* i preuzmete stabilnu
virtualnu mašinu kod kuće, da ne biste trošili mobilni internet putem hotspota.

**Napomena 2**: Moguće je da na fakultetskim računarima u NTP-u nije instaliran *Launcher*.
Ovo se lako možete rešiti prateće uputstvo iznad.

