use num_derive::FromPrimitive;
use serde::{Serialize, Deserialize};
use mona_derive::{TargetFunctionData, EnumLen};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize)]
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[derive(TargetFunctionData, EnumLen, FromPrimitive, Display, EnumIter)]
pub enum TargetFunctionName {
    MaxATK,
    MaxDEF,
    MaxHP,
    MaxEM,
    MaxRecharge,
    PyroDamage,
    CryoDamage,
    HydroDamage,
    ElectroDamage,
    AnemoDamage,
    DendroDamage,
    GeoDamage,
    PhysicalDamage,
    MaxVaporize,
    MaxMelt,
    ExpectVaporize,
    ExpectMelt,

    AlbedoDefault,
    AloyDefault,
    AmberDefault,
    AratakiIttoDefault,
    BarbaraDefault,
    BeidouDefault,
    BennettDamage,
    BennettDefault,
    ChongyunDefault,
    DilucDefault,
    DionaDefault,
    EulaDefault,
    FischlDefault,
    GanyuDefault,
    GorouDefault,
    HuTaoDefault,
    JeanDefault,
    KaedeharaKazuhaDamage,
    KaedeharaKazuhaDefault,
    KaeyaDefault,
    KamisatoAyakaDefault,
    KamisatoAyakaDps,
    KamisatoAyatoDefault,
    KeqingDefault,
    KleeDefault,
    KujouSaraDamage,
    KujouSaraDefault,
    LisaDefault,
    MonaDefault,
    NingguangDefault,
    NoelleDefault,
    QiqiDefault,
    RaidenShogunDefault,
    RazorDefault,
    RosariaDefault,
    SangonomiyaKokomiDefault,
    SayuDefault,
    ShenheDefault,
    SucroseDefault,
    TartagliaDefault,
    ThomaDefault,
    VentiDefault,
    XianglingDefault,
    XiaoDefault,
    XingqiuDefault,
    XinyanDamage,
    XinyanDefault,
    YaeMikoDefault,
    YanfeiDefault,
    YelanDefault,
    YoimiyaDefault,
    YunjinDefault,
    ZhongliDefault,
    KukiShinobuDefault,
    ShikanoinHeizouDefault,
    TighnariDefault,
    CynoDefault,
    NilouDefault,
    NahidaDefault,
    WandererDefault,
    FaruzanDamage,
    AlhaithamDefault,
    DehyaDefault,
    MikaDefault,
    FreminetDefault,
    LyneyDefault,
}
