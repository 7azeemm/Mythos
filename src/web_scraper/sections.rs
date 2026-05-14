use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::{Database, Postgres};

macro_rules! define_sections {
    (
        $(
            $variant:ident => ($name:expr, $requires_desc:expr, $parent:expr)
        ),* $(,)?
    ) => {
        #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Section {
            $( $variant ),*
        }

        impl Section {
            pub const ALL: &'static [Section] = &[
                $( Section::$variant ),*
            ];

            pub fn to_str(&self) -> &'static str {
                match self {
                    $( Section::$variant => $name ),*
                }
            }

            pub fn from_str(s: &str) -> Option<Self> {
                match s {
                    $( $name => Some(Section::$variant), )*
                    _ => None,
                }
            }

            pub fn requires_description(&self) -> bool {
                match self {
                    $( Section::$variant => $requires_desc ),*
                }
            }

            pub fn parent(&self) -> Option<Section> {
                match self {
                    $(
                        Section::$variant => $parent,
                    )*
                }
            }

            pub fn children(&self) -> Vec<Section> {
                let mut children = vec![*self];
                $(
                    let parent_opt: Option<Section> = $parent;
                    if let Some(parent) = parent_opt {
                        if parent == *self {
                            children.push(Section::$variant);
                        }
                    }
                )*
                children
            }
        }
    };
}

define_sections! {
    PC           => ("pc", false, None),
    GamingPC     => ("gaming_pc", false, Some(Section::PC)),
    AllInOnePC   => ("pc_all_in_one", false, Some(Section::PC)),
    GamingSetup  => ("gaming_setup", false, Some(Section::PC)),

    Laptop       => ("laptop", false, None),
    GamingLaptop => ("gaming_laptop", false, Some(Section::Laptop)),
    ProLaptop    => ("pro_laptop", false, Some(Section::Laptop)),
    MacBook      => ("macbook", false, Some(Section::Laptop)),

    Monitor      => ("monitor", false, None),
    Mouse        => ("mouse", false, None),
    KeyBoard     => ("keyboard", false, None),

    CPU          => ("cpu", false, None),
    GPU          => ("gpu", false, None),
    RAM          => ("ram", false, None),
    MotherBoard  => ("motherboard", false, None),
    Storage      => ("storage", false, None),
    SSD          => ("ssd", false, Some(Section::Storage)),
    NVMe         => ("nvme", false, Some(Section::Storage)),
    HDD          => ("hdd", false, Some(Section::Storage)),
    Case         => ("case", false, None),
    PSU          => ("psu", false, None),
    Cooler       => ("cooler", false, None),
    AirCooler    => ("air_cooler", false, Some(Section::Cooler)),
    WaterCooler  => ("water_cooler", false, Some(Section::Cooler)),
    Fan          => ("fan", false, Some(Section::Cooler)),
}

impl sqlx::Type<Postgres> for Section {
    fn type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as sqlx::Type<Postgres>>::type_info()
    }
}

impl<'q> sqlx::Encode<'q, Postgres> for Section {
    fn encode_by_ref(&self, buf: &mut <Postgres as Database>::ArgumentBuffer) -> Result<IsNull, BoxDynError> {
        <&str as sqlx::Encode<'_, Postgres>>::encode_by_ref(&self.to_str(), buf)
    }
}

impl<'r> sqlx::Decode<'r, Postgres> for Section {
    fn decode(value: <Postgres as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        let s = <&str as sqlx::Decode<'r, Postgres>>::decode(value)?;
        Section::from_str(s).ok_or_else(|| format!("Unknown section: {}", s).into())
    }
}

impl sqlx::postgres::PgHasArrayType for Section {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <&str as sqlx::postgres::PgHasArrayType>::array_type_info()
    }
}