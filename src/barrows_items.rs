
#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Debug)]
#[repr(u8)]
pub enum Brother {
    Ahrim  = 0b0010_0000u8,
    Dharok = 0b0100_0000u8,
    Guthan = 0b0110_0000u8,
    Karil  = 0b1000_0000u8,
    Torag  = 0b1010_0000u8,
    Verac  = 0b1100_0000u8,
}
impl Brother {
    pub fn get_items(&self) -> [Item;4] {
        match self {
            &Self::Ahrim => {
                [
                    Item::AhrimsHood,
                    Item::AhrimsRobeTop,
                    Item::AhrimsSkirt,
                    Item::AhrimsStaff,
                ]
            }
            &Self::Dharok => {
                [
                    Item::DharoksHelm,
                    Item::DharoksPlateBody,
                    Item::DharoksPlateLegs,
                    Item::DharoksGreatAxe,
                ]
            }
            &Self::Guthan => {
                [
                    Item::GuthansHelm,
                    Item::GuthansPlatebody,
                    Item::GuthansChainskirt,
                    Item::GuthansWarspear,
                ]
            }
            &Self::Karil => {
                [
                    Item::KarilsCoif,
                    Item::KarilsLeatherTop,
                    Item::KarilsLeatherSkirt,
                    Item::KarilsCrossbow,
                ]
            }
            &Self::Torag => {
                [
                    Item::ToragsHelm,
                    Item::ToragsPlateBody,
                    Item::ToragsPlateLegs,
                    Item::ToragsHammers,
                ]
            }
            &Self::Verac => {
                [
                    Item::VeracsHelm,
                    Item::VeracsBrassard,
                    Item::VeracsPlateskirt,
                    Item::VeracsFlail,
                ]
            }
        }
    }
}

#[test]
fn assert_stuff_about_size() {
    assert_eq!( std::mem::size_of::<Option<Brother>>(), std::mem::size_of::<u8>());
    assert_eq!( std::mem::size_of::<Option<Item>>(), std::mem::size_of::<u8>());
}

#[derive(Clone,Copy,PartialEq,Eq,PartialOrd,Ord,Debug)]
#[repr(u8)]
pub enum Item {
    AhrimsHood         = 0b0010_0000u8 + 0u8,
    AhrimsRobeTop      = 0b0010_0000u8 + 1u8,
    AhrimsSkirt        = 0b0010_0000u8 + 2u8,
    AhrimsStaff        = 0b0010_0000u8 + 3u8,

    DharoksHelm        = 0b0100_0000u8 + 4u8,
    DharoksPlateBody   = 0b0100_0000u8 + 5u8,
    DharoksPlateLegs   = 0b0100_0000u8 + 6u8,
    DharoksGreatAxe    = 0b0100_0000u8 + 7u8,

    GuthansHelm        = 0b0110_0000u8 + 8u8,
    GuthansPlatebody   = 0b0110_0000u8 + 9u8,
    GuthansChainskirt  = 0b0110_0000u8 + 10u8,
    GuthansWarspear    = 0b0110_0000u8 + 11u8,

    KarilsCoif         = 0b1000_0000u8 + 12u8,
    KarilsLeatherTop   = 0b1000_0000u8 + 13u8,
    KarilsLeatherSkirt = 0b1000_0000u8 + 14u8,
    KarilsCrossbow     = 0b1000_0000u8 + 15u8,

    ToragsHelm         = 0b1010_0000u8 + 16u8,
    ToragsPlateBody    = 0b1010_0000u8 + 17u8,
    ToragsPlateLegs    = 0b1010_0000u8 + 18u8,
    ToragsHammers      = 0b1010_0000u8 + 19u8,

    VeracsHelm         = 0b1100_0000u8 + 20u8,
    VeracsBrassard     = 0b1100_0000u8 + 21u8,
    VeracsPlateskirt   = 0b1100_0000u8 + 22u8,
    VeracsFlail        = 0b1100_0000u8 + 23u8,
}
impl std::fmt::Display for Item {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //use std::fmt::Write;
        fmt.write_str(self.as_str())
    }
}
impl Item {
    pub fn as_str(&self) -> &'static str {
        match self {
            Item::AhrimsHood => "AhrimsHood",
            Item::AhrimsRobeTop => "AhrimsRobeTop",
            Item::AhrimsSkirt => "AhrimsSkirt",
            Item::AhrimsStaff => "AhrimsStaff",
            Item::DharoksHelm => "DharoksHelm",
            Item::DharoksPlateBody => "DharoksPlateBody",
            Item::DharoksPlateLegs => "DharoksPlateLegs",
            Item::DharoksGreatAxe => "DharoksGreatAxe",
            Item::GuthansHelm => "GuthansHelm",
            Item::GuthansPlatebody => "GuthansPlatebody",
            Item::GuthansChainskirt => "GuthansChainskirt",
            Item::GuthansWarspear => "GuthansWarspear",
            Item::KarilsCoif => "KarilsCoif",
            Item::KarilsLeatherTop => "KarilsLeatherTop",
            Item::KarilsLeatherSkirt => "KarilsLeatherSkirt",
            Item::KarilsCrossbow => "KarilsCrossbow",
            Item::ToragsHelm => "ToragsHelm",
            Item::ToragsPlateBody => "ToragsPlateBody",
            Item::ToragsPlateLegs => "ToragsPlateLegs",
            Item::ToragsHammers => "ToragsHammers",
            Item::VeracsHelm => "VeracsHelm",
            Item::VeracsBrassard => "VeracsBrassard",
            Item::VeracsPlateskirt => "VeracsPlateskirt",
            Item::VeracsFlail => "VeracsFlail",
        }
    }

    #[allow(dead_code)]
    pub fn get_index(&self) -> usize {
        let index = self.clone() as usize & 0b0001_1111usize;
        if index >= 24 {
            panic!("index: {:?} should not exceed 24, it did for item: {:?}", self, self)
        } else {
            index
        }
    }

    #[allow(dead_code)]
    pub fn get_brother(&self) -> Brother {
        let brother: u8 = self.clone() as u8 & 0b1110_0000u8;
        unsafe { std::mem::transmute::<u8,Brother>(brother) }
    }
}

pub const ITEMS: [Item;24] = [
    Item::AhrimsHood,
    Item::AhrimsRobeTop,
    Item::AhrimsSkirt,
    Item::AhrimsStaff,
    Item::DharoksHelm,
    Item::DharoksPlateBody,
    Item::DharoksPlateLegs,
    Item::DharoksGreatAxe,
    Item::GuthansHelm,
    Item::GuthansPlatebody,
    Item::GuthansChainskirt,
    Item::GuthansWarspear,
    Item::KarilsCoif,
    Item::KarilsLeatherTop,
    Item::KarilsLeatherSkirt,
    Item::KarilsCrossbow,
    Item::ToragsHelm,
    Item::ToragsPlateBody,
    Item::ToragsPlateLegs,
    Item::ToragsHammers,
    Item::VeracsHelm,
    Item::VeracsBrassard,
    Item::VeracsPlateskirt,
    Item::VeracsFlail,
];

#[test]
fn assert_item_at_index() {
    for i in 0..24 {
        let item = ITEMS[i].clone();
        let index: usize = item.get_index();
        assert_eq!(index, i, "Failed for Item:'{:?} (0b{:04b}_{:04b})'", item, (item as u8 >>4), item as u8 & 0b0000_1111u8);
        match i {
            0..=3 => assert_eq!(item.get_brother(), Brother::Ahrim),
            4..=7 => assert_eq!(item.get_brother(), Brother::Dharok),
            8..=11=> assert_eq!(item.get_brother(), Brother::Guthan),
            12..=15 => assert_eq!(item.get_brother(), Brother::Karil),
            16..=19 => assert_eq!(item.get_brother(), Brother::Torag),
            20..=23 => assert_eq!(item.get_brother(), Brother::Verac),
            _ => panic!("out of bounds"),
        }
    }
}

