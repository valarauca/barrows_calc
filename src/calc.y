%start GetPredicate
%left AND
%left OR
%%
GetPredicate -> Result<Predicate,()>:
  'LPAREN' GetPredicate 'RPAREN' { Ok($2?) } 
  | GetPredicate 'AND' GetPredicate { Ok(Predicate::And(Box::new($1?),Box::new($3?))) }
  | GetPredicate 'OR' GetPredicate { Ok(Predicate::Or(Box::new($1?),Box::new($3?))) }
  | GetItem { Ok(Predicate::Item($1?)) }
  ;

GetItem -> Result<Item,()>:
  'AhrimsHood' { Ok(Item::AhrimsHood) }
  | 'AhrimsRobeTop' { Ok(Item::AhrimsRobeTop) }
  | 'AhrimsSkirt' { Ok(Item::AhrimsSkirt) }
  | 'AhrimsStaff' { Ok(Item::AhrimsStaff) }
  | 'DharoksHelm' { Ok(Item::DharoksHelm) }
  | 'DharoksPlateBody' { Ok(Item::DharoksPlateBody) }
  | 'DharoksPlateLegs' { Ok(Item::DharoksPlateLegs) }
  | 'DharoksGreatAxe' { Ok(Item::DharoksGreatAxe) }
  | 'GuthansHelm' { Ok(Item::GuthansHelm) }
  | 'GuthansPlatebody' { Ok(Item::GuthansPlatebody) }
  | 'GuthansChainskirt' { Ok(Item::GuthansChainskirt) }
  | 'GuthansWarspear' { Ok(Item::GuthansWarspear) }
  | 'KarilsCoif' { Ok(Item::KarilsCoif) }
  | 'KarilsLeatherTop' { Ok(Item::KarilsLeatherTop) }
  | 'KarilsLeatherSkirt' { Ok(Item::KarilsLeatherSkirt) }
  | 'KarilsCrossbow' { Ok(Item::KarilsCrossbow) }
  | 'ToragsHelm' { Ok(Item::ToragsHelm) }
  | 'ToragsPlateBody' { Ok(Item::ToragsPlateBody) }
  | 'ToragsPlateLegs' { Ok(Item::ToragsPlateLegs) }
  | 'ToragsHammers' { Ok(Item::ToragsHammers) }
  | 'VeracsHelm' { Ok(Item::VeracsHelm) }
  | 'VeracsBrassard' { Ok(Item::VeracsBrassard) }
  | 'VeracsPlateskirt' { Ok(Item::VeracsPlateskirt) }
  | 'VeracsFlail' { Ok(Item::VeracsFlail) }
  ;

%%
use crate::clog::Predicate;
use crate::barrows_items::Item;
