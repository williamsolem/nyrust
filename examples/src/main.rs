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

    offentleg(cagette) funksjon kanskje(å: u32) -> Kanskje<Resultat<u32, Streng>> {
        viss å % 2 == 1 {
            viss å == 42 {
                Noko(Feil(Streng::fra("helvete")))
            } elles {
                Noko(Ok(33))
            }
        } elles {
            Ingen
        }
    }

    asynkron funksjon døme() {
    }

    asynkron funksjon døme2() {
        døme().attend;
    }

    funksjon hovud() {
        la muterbar x = 31;

        jamstill x {
            42 => {
                skrivln!("Pølse i vaffel")
            }
            _ => skrivln!("Det finnes ikkje dårleg ver, bare dårlege klede")
        }

        for æ i 0..10 {
            la val = løkke {
                bryt æ;
            };

            mens kva x < val {
                x += 1;
            }

            x = viss la Noko(resultat) = kanskje(i) {
                resultat.pakk_ut()
            } elles {
                12
            };
        }

        //sekundær();
    }

    #[godta(unåeleg_kode)]
    funksjon sekundær() {
        panikk!("faen"); // for the true norwegian experience
        faen!("hestkuk"); // for friends speaking northern dialects of norwegian
        oisann!("fy søren"); // in SFW contexts
    }
}
