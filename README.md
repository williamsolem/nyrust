# nyrust

![](logo.jpeg)

Aren't you _dritt lei_ from writing Rust programs in English? Do you like saying
"faen" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some French touch to your
programs?

**nyrust** (A combination of norwegian for _Rust_, which is exactly the same as in english, and _ny_ as in _nynorsk_) is here to save your day, as it allows you to
write Rust programs in Norwegian (nynorsk), using nynorsk keywords, nynorsk function names,
nynorsk idioms.

This has been designed to be used as the official programming language to
develop the future Bergen sovereign operating system. 

If you're from Bergen or any other county with nynorsk as an official 
language: I will be awaiting your donations on
[skatteetaten.no](https://skatteetaten.no/).

You're from a part of Norway where people actually live (or elsewhere), and don't feel at ease using only nynorsk words? 

Don't worry!
Nynorsk Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Nyrust:

### trait and impl (aka grensesnitt og oppfylling)

```rust
nyrust::nyrust! {
    ekstern kasse nyrust;

    bruk std::collections::Ordbok som Bok;

    oppfyll NøkkelVerdi {
        funksjon skriv(&sjølv, nøkkel: Streng, verdi: Streng);
        funksjon hent(&sjølv, nøkkel: Streng) -> Resultat<Kanskje<&Streng>, Streng>;
    }

    statisk muterbar ORDBOK: Kanskje<Bok<Streng, Streng>> = Ingen;

    struktur Konkret;

    oppfyll NøkkelVerdi for Konkret {
        funksjon skriv(&sjølv, nøkkel: Streng, verdi: Streng) {
            la Ordbok = utrygg {
                ORDBOK.hent_eller_tilbakefall_til(Défaut::défaut)
            };
            Ordbok.sett_inn(nøkkel, verdi);
        }
        funksjon hent(&sjølv, nøkkel: Streng) -> Resultat<Kanskje<&Streng>, Streng> {
            viss la Noko(Ordbok) = utrygg { ORDBOK.som_referanse() } {
                Ok(Ordbok.hent(&nøkkel))
            } elles {
                Feil("Fann ikkje i ordbok".til())
            }
        }
    }
}
```

### Support for regional languages

```rust
#[godta(unåeleg_kode)]
funksjon sekundær() {
    panikk!("faen"); // for the true norwegian experience
    faen!("hestkuk"); // for friends speaking northern dialects of norwegian
    oisann!("fy søren"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Vipps (not the company), that's it.

## Bidra

First of all, _takk skal du ha_ for considering participating to this joke, the
Bergen government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `hovud` (Nynorsk for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your norwegian.

## but kvifor would you do det?

- to make Ivar Aasen proud
- have the chance of getting some kind of award from Språkrådet
- stop the swedes from teasing us because we don't have a localized version of Rust
- vere ein del av gutta
- guttastemning
- fortele hele verda at du er frå Norge

## Other languages

- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
- Croatian: [hrđa](https://github.com/njelich/hrdja)

## En stor takk til

- Meg 😎

## Lisensen

kven bryr seg om lisens, ærleg talt?
