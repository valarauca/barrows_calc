# Barrows Calc

Calculates KC to "complete" some milestone.

```
OPTIONS:
    -b, --brothers <brothers>...
            which brothers are you slaying [default: all]  [possible values: all, ahrim, dharok, guthan, karil, torag,
            verac]
    -m, --leagues-modifier <LEAGUES_MODIFIER>    multipler for leagues [default: 1]
    -s, --sigma <SIGMA>                          how many standard deviations do you want to graph [default: 3]
        --threads <THREADS>                      state number of threads to use [default: 4]
        --trials <THREADS>                       number of trails to run [default: 1000000]
    -w, --want <want>                            what do you want
```

# Wants

How do tell the tool what you want?

```
Predicate := Item
             | Predicate '&' Predicate
             | Predicate '|' Predicate
             | '(' Predicate ')'
             ;

Item := AhrimsHood
        | AhrimsRobeTop
        | AhrimsSkirt
        | AhrimsStaff
        | DharoksHelm
        | DharoksPlateBody
        | DharoksPlateLegs
        | DharoksGreatAxe
        | GuthansHelm
        | GuthansPlatebody
        | GuthansChainskirt
        | GuthansWarspear
        | KarilsCoif
        | KarilsLeatherTop
        | KarilsLeatherSkirt
        | KarilsCrossbow
        | ToragsHelm
        | ToragsPlateBody
        | ToragsPlateLegs
        | ToragsHammers
        | VeracsHelm
        | VeracsBrassard
        | VeracsPlateskirt
        | VeracsFlail
        ;
```

## Bro, I play runescape I don't understand what Extended Backus-Naur Form Is

Say you want tank legs

```
--want 'ToragsPlateLegs|DharoksPlateLegs|VeracsPlateskirt|GuthansChainskirt'
```

Say you want full ahrims

```
--want 'AhrimsRobeTop&AhrimsSkirt&AhrimsStaff&AHrimsHood'
```

Say you want "The Iron Special"

* Tank Legs
* Karils top & bottoms
* Ahrims top, bottom, and staff

```
--want '(ToragsPlateLegs|DharoksPlateLegs|VeracsPlateskirt|GuthansChainskirt)&(AhrimsRobeTop&AhrimsSkirt&AhrimsStaff)&(KarilsLeatherTop&KarilsLeatherSkirt)'
```

See the pattern?
