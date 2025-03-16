rooste::rooste! {
    väline teek rooste;

    kasuta std::kollektsioonid::Paisktabel nagu Register;

    omadus Nimistu {
        funktsioon sisesta(&ise, võti: Sõne, väärtus: Sõne);
        funktsioon väljasta(&ise, võti: Sõne) -> Tulemus<Võimalik<&Sõne>, Sõne>;
    }

    staatiline muutuv REGISTER: Võimalik<Register<Sõne, Sõne>> = Puudu;

    teostus dün Nimistu {
        funktsioon kirjuta(&ise, võti: Sõne, väärtus: Sõne) {
            on register = ebaturvaline {
                REGISTER.võta_või_sisesta_koos(Vaikimisi::vaikimisi)
            };

            register.sisesta(võti, väärtus);
        }

        funktsioon väljasta(&ise, võti: Sõne) -> Tulemus<Võimalik<&Sõne>, Sõne> {
            kui on Olemas(register) = ebaturvaline { REGISTER.viitena() } {
                Okei(register.võta(&võti))
            } muidu {
                Viga("loo nimistu".muunda())
            }
        }
    }

    avalik(teek) funktsioon võib_olla(i: u32) -> Võimalik<Tulemus<u32, Sõne>> {
        kui i % 2 == 1 {
            kui i == 42 {
                Olemas(Viga(Sõne::loo("jama")))
            } muidu {
                Olemas(Okei(33))
            }
        } muidu {
            Puudu
        }
    }

    asünk funktsioon näide() {
    }

    asünk funktsioon näide2() {
        näide().oota;
    }

    funktsioon peamine() {
        on muutuv x = 31;

        sobita x {
            42 => {
                trüki!("kiluvõileib")
            }
            _ => trüki!("vaat nii")
        }

        iga i olles 0..10 {
            on väärtus = kordus {
                katke i;
            };

            kuni x < väärtus {
                x += 1;
            }

            x = kui on Olemas(tulemus) = võib_olla(i) {
                tulemus.ava()
            } muidu {
                12
            };
        }

        // sekundaarne();
    }

    #[luba(kättesaamatu_kood)]
    funktsioon sekundaarne() {
        raisk!("oh ei"); // for the true Estonian experience
        paanika!("see juhtus"); // for more polite contexts
        ups!("siin läks pekki"); // in SFW contexts
    }
}
