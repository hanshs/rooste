# Rooste

![](https://github.com/bnjbvr/rooste/raw/principale/logo.jpeg)

Aren't you _vässu_ from writing Rust programs in English? Do you like saying
"raisk" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Estonian touch to your
programs?

**rooste** (Estonian for _Rust_) is here to save your day, as it allows you to
write Rust programs in Estonian, using Estonian keywords, Estonian function names,
Estonian idioms.

This has been designed to be used as the official programming language to
develop the future Estonian sovereign operating system.

You're from Quebec (or elsewhere) and don't feel at ease using only Estonian words?

Don't worry!
Estonian Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with Rooste:

### trait and impl (aka omadus ja teostus)

```rust
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
}
```

### Support for various contexts

```rust
#[luba(kättesaamatu_kood)]
funktsioon sekundaarne() {
    raisk!("oh ei"); // for the true Estonian experience
    paanika!("see juhtus"); // for more polite contexts
    ups!("siin läks pekki"); // in SFW contexts
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. _Vot nii_, that's it.

## les contributions

First of all, _tänks_ for considering participating to this joke, the
Estonian government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `peamine` (Estonian for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your Estonian.

## but why

- horsin around
- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.
- sa igatsed seda eestikeelset progemist mida viimati nähti tehnika_ylikoolis()?

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
- Persian: [zangar (زنگار)](https://github.com/ui-ce/zangar)
- Malagasy: [arafesina](https://github.com/luckasRanarison/arafesina)
- Latin: [ferrugo](https://github.com/pianoman911/ferrugo)
- Norwegian: [korrosjon](https://github.com/datagutt/korrosjon)
- All of the above: [unirust](https://github.com/charyan/unirust)

## TODO
- [ ] More Estonian Logo
