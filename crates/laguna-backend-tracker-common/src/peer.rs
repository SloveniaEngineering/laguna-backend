use std::{
    fmt,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
};

use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::Bytes;

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct PeerId(
    // OLD: See: https://github.com/serde-rs/bytes/pull/28
    // #[serde(with = "serde_byte_array")]
    #[serde_as(as = "Bytes")] pub [u8; 20],
);

/// Peer client identification enum.
/// Resources:
/// - <https://wiki.theory.org/BitTorrentSpecification>
/// - <https://github.com/torrust/torrust-tracker/blob/develop/src/tracker/peer.rs>
/// - <https://github.com/kenpaicat/aquatic/blob/master/aquatic_peer_id/src/lib.rs>
#[derive(Debug)]
pub enum PeerClient {
    AnyEventBitTorrent,
    Arctic,
    Ares,
    Artemis,
    ATorrentForAndroid,
    Avicora,
    Azureus,
    BareTorrent,
    BitBuddy,
    BitComet,
    BitCometLightOrBitBlinder,
    Bitflu,
    BitLet,
    BitPump,
    BitRocket,
    BitSpirit,
    BitTorrent,
    BitTorrentPro,
    BittorrentX,
    BitWombat,
    Bt,
    BTG,
    BTSlave,
    CTorrent,
    DelugeTorrent,
    EBit,
    ElectricSheep,
    EnhancedCTorrent,
    FileCroc,
    FireTorrent,
    FoxTorrent,
    FreeboxBitTorrent,
    FreeDownloadManager,
    FrostWire,
    GSTorrent,
    Halite,
    Hekate,
    HMule,
    Hydranode,
    ILivid,
    JustSeedIt,
    KGet,
    KTorrent,
    LeechCraft,
    LHABC,
    LibTorrent,
    LibTorrentTheOtherOne,
    LimeWire,
    Lphant,
    Mainline,
    MainlineBitTorrentOrBBTor,
    Meerkat,
    Miro,
    MonoTorrent,
    MoonlightTorrent,
    MooPolice,
    NetBitTorrent,
    NetTransport,
    OmegaTorrent,
    OneSwarm,
    Pando,
    PHPTracker,
    PicoTorrent,
    PropagateDataClient,
    ProtocolBitTorrent,
    Q4Torrent,
    QBittorrent,
    QDownload,
    Retriever,
    RezTorrent,
    Shareaza,
    ShareazaAlphaOrBeta,
    SharkTorrent,
    SoMud,
    SwarmScope,
    Swiftbit,
    SymTorrent,
    TerasaurSeedBank,
    Thunder,
    Torch,
    TorrentDotNET,
    Torrentstorm,
    Transmission,
    Tribler,
    TuoTu,
    ULeecher,
    UTorrent,
    UTorrentEmbedded,
    UTorrentMac,
    UTorrentWeb,
    Vagaa,
    WebTorrent,
    WebTorrentDesktop,
    WeirdChineseSussyBaka,
    WeirdSussyBaka,
    XanTorrent,
    XFPlay,
    XSwifter,
    XTorrent,
    Xunlei,
    ZipTorrent,
}

pub enum PeerClientError {
    UnknownClient,
}

impl TryFrom<[u8; 2]> for PeerClient {
    type Error = PeerClientError;
    fn try_from(name: [u8; 2]) -> Result<Self, Self::Error> {
        match &name {
            b"7T" => Ok(PeerClient::ATorrentForAndroid),
            b"AB" => Ok(PeerClient::AnyEventBitTorrent),
            b"AG" | b"A~" => Ok(PeerClient::Ares),
            b"AR" => Ok(PeerClient::Arctic),
            b"AT" => Ok(PeerClient::Artemis),
            b"AV" => Ok(PeerClient::Avicora),
            b"AX" => Ok(PeerClient::BitPump),
            b"AZ" => Ok(PeerClient::Azureus),
            b"BB" => Ok(PeerClient::BitBuddy),
            b"BC" => Ok(PeerClient::BitComet),
            b"BD" => Ok(PeerClient::WeirdSussyBaka),
            b"BE" => Ok(PeerClient::BareTorrent),
            b"BF" => Ok(PeerClient::Bitflu),
            b"BG" => Ok(PeerClient::BTG),
            b"BL" => Ok(PeerClient::BitCometLightOrBitBlinder),
            b"BP" => Ok(PeerClient::BitTorrentPro),
            b"BR" => Ok(PeerClient::BitRocket),
            b"BS" => Ok(PeerClient::BTSlave),
            b"Bt" => Ok(PeerClient::Bt),
            b"BT" => Ok(PeerClient::MainlineBitTorrentOrBBTor),
            b"BW" => Ok(PeerClient::BitWombat),
            b"BX" => Ok(PeerClient::BittorrentX),
            b"CD" => Ok(PeerClient::EnhancedCTorrent),
            b"CT" => Ok(PeerClient::CTorrent),
            b"DE" => Ok(PeerClient::DelugeTorrent),
            b"DP" => Ok(PeerClient::PropagateDataClient),
            b"EB" => Ok(PeerClient::EBit),
            b"ES" => Ok(PeerClient::ElectricSheep),
            b"FC" => Ok(PeerClient::FileCroc),
            b"FD" => Ok(PeerClient::FreeDownloadManager),
            b"FT" => Ok(PeerClient::FoxTorrent),
            b"FW" => Ok(PeerClient::FrostWire),
            b"FX" => Ok(PeerClient::FreeboxBitTorrent),
            b"GS" => Ok(PeerClient::GSTorrent),
            b"HK" => Ok(PeerClient::Hekate),
            b"hk" => Ok(PeerClient::WeirdChineseSussyBaka),
            b"HL" => Ok(PeerClient::Halite),
            b"HM" => Ok(PeerClient::HMule),
            b"HN" => Ok(PeerClient::Hydranode),
            b"iL" => Ok(PeerClient::ILivid),
            b"JS" => Ok(PeerClient::JustSeedIt),
            b"KG" => Ok(PeerClient::KGet),
            b"KT" => Ok(PeerClient::KTorrent),
            b"LC" => Ok(PeerClient::LeechCraft),
            b"LH" => Ok(PeerClient::LHABC),
            b"LP" => Ok(PeerClient::Lphant),
            b"LT" => Ok(PeerClient::LibTorrent),
            b"lt" => Ok(PeerClient::LibTorrentTheOtherOne),
            b"LW" => Ok(PeerClient::LimeWire),
            b"MK" => Ok(PeerClient::Meerkat),
            b"MO" => Ok(PeerClient::MonoTorrent),
            b"MP" => Ok(PeerClient::MooPolice),
            b"MR" => Ok(PeerClient::Miro),
            b"MT" => Ok(PeerClient::MoonlightTorrent),
            b"NB" => Ok(PeerClient::NetBitTorrent),
            b"NP" => Ok(PeerClient::WeirdSussyBaka),
            b"NT" | b"NX" => Ok(PeerClient::NetTransport),
            b"OS" => Ok(PeerClient::OneSwarm),
            b"OT" => Ok(PeerClient::OmegaTorrent),
            b"PB" => Ok(PeerClient::ProtocolBitTorrent),
            b"PD" => Ok(PeerClient::Pando),
            b"PI" => Ok(PeerClient::PicoTorrent),
            b"PT" => Ok(PeerClient::PHPTracker),
            b"qB" => Ok(PeerClient::QBittorrent),
            b"QD" => Ok(PeerClient::QDownload),
            b"QT" => Ok(PeerClient::Q4Torrent),
            b"RT" => Ok(PeerClient::Retriever),
            b"RZ" => Ok(PeerClient::RezTorrent),
            b"S~" => Ok(PeerClient::ShareazaAlphaOrBeta),
            b"SB" => Ok(PeerClient::Swiftbit),
            b"SD" => Ok(PeerClient::Thunder),
            b"SM" => Ok(PeerClient::SoMud),
            b"SP" => Ok(PeerClient::BitSpirit),
            b"SS" => Ok(PeerClient::SwarmScope),
            b"st" => Ok(PeerClient::SharkTorrent),
            b"ST" => Ok(PeerClient::SymTorrent),
            b"SZ" => Ok(PeerClient::Shareaza),
            b"TB" => Ok(PeerClient::Torch),
            b"TE" => Ok(PeerClient::TerasaurSeedBank),
            b"TL" => Ok(PeerClient::Tribler),
            b"TN" => Ok(PeerClient::TorrentDotNET),
            b"TR" => Ok(PeerClient::Transmission),
            b"TS" => Ok(PeerClient::Torrentstorm),
            b"TT" => Ok(PeerClient::TuoTu),
            b"UE" => Ok(PeerClient::UTorrentEmbedded),
            b"UL" => Ok(PeerClient::ULeecher),
            b"UM" => Ok(PeerClient::UTorrentMac),
            b"UT" => Ok(PeerClient::UTorrent),
            b"UW" => Ok(PeerClient::UTorrentWeb),
            b"VG" => Ok(PeerClient::Vagaa),
            b"WD" => Ok(PeerClient::WebTorrentDesktop),
            b"wF" => Ok(PeerClient::WeirdSussyBaka),
            b"WT" => Ok(PeerClient::BitLet),
            b"WW" => Ok(PeerClient::WebTorrent),
            b"WY" => Ok(PeerClient::FireTorrent),
            b"XF" => Ok(PeerClient::XFPlay),
            b"XL" => Ok(PeerClient::Xunlei),
            b"XS" => Ok(PeerClient::XSwifter),
            b"XT" => Ok(PeerClient::XanTorrent),
            b"XX" => Ok(PeerClient::XTorrent),
            b"ZT" => Ok(PeerClient::ZipTorrent),
            _ => Err(PeerClientError::UnknownClient),
        }
    }
}

impl fmt::Display for PeerClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PeerClient::AnyEventBitTorrent => write!(f, "AnyEvent::BitTorrent"),
            PeerClient::Arctic => write!(f, "Arctic"),
            PeerClient::Ares => write!(f, "Ares"),
            PeerClient::Artemis => write!(f, "Artemis"),
            PeerClient::ATorrentForAndroid => write!(f, "aTorrent for Android"),
            PeerClient::Avicora => write!(f, "Avicora"),
            PeerClient::Azureus => write!(f, "Azureus"),
            PeerClient::BareTorrent => write!(f, "BareTorrent"),
            PeerClient::BitBuddy => write!(f, "BitBuddy"),
            PeerClient::BitComet => write!(f, "BitComet"),
            PeerClient::BitCometLightOrBitBlinder => write!(f, "BitComet Light/BitBlinder"),
            PeerClient::Bitflu => write!(f, "Bitflu"),
            PeerClient::BitLet => write!(f, "BitLet"),
            PeerClient::BitPump => write!(f, "BitPump"),
            PeerClient::BitRocket => write!(f, "BitRocket"),
            PeerClient::BitSpirit => write!(f, "BitSpirit"),
            PeerClient::BitTorrent => write!(f, "BitTorrent"),
            PeerClient::BitTorrentPro => write!(f, "BitTorrent Pro"),
            PeerClient::BittorrentX => write!(f, "~BitTorrent X"),
            PeerClient::BitWombat => write!(f, "BitWombat"),
            PeerClient::Bt => write!(f, "BT"),
            PeerClient::BTG => write!(f, "BTG (Rasterbar libtorrent)"),
            PeerClient::BTSlave => write!(f, "BTSlave"),
            PeerClient::CTorrent => write!(f, "CTorrent"),
            PeerClient::DelugeTorrent => write!(f, "DelugeTorrent"),
            PeerClient::EBit => write!(f, "EBit"),
            PeerClient::ElectricSheep => write!(f, "Electric Sheep"),
            PeerClient::EnhancedCTorrent => write!(f, "Enhanced CTorrent"),
            PeerClient::FileCroc => write!(f, "FileCroc"),
            PeerClient::FireTorrent => write!(f, "FireTorrent"),
            PeerClient::FoxTorrent => write!(f, "FoxTorrent"),
            PeerClient::FreeboxBitTorrent => write!(f, "Freebox BitTorrent"),
            PeerClient::FreeDownloadManager => write!(f, "Free Download Manager"),
            PeerClient::FrostWire => write!(f, "FrostWire"),
            PeerClient::GSTorrent => write!(f, "GSTorrent"),
            PeerClient::Halite => write!(f, "Halite"),
            PeerClient::Hekate => write!(f, "Hekate"),
            PeerClient::HMule => write!(f, "hMule"),
            PeerClient::Hydranode => write!(f, "Hydranode"),
            PeerClient::ILivid => write!(f, "iLivid"),
            PeerClient::JustSeedIt => write!(f, "JustSeed.it"),
            PeerClient::KGet => write!(f, "KGet"),
            PeerClient::KTorrent => write!(f, "KTorrent"),
            PeerClient::LeechCraft => write!(f, "LeechCraft"),
            PeerClient::LHABC => write!(f, "LH-ABC"),
            PeerClient::LibTorrent => write!(f, "LibTorrent"),
            PeerClient::LibTorrentTheOtherOne => write!(f, "libtorrent (the other one)"),
            PeerClient::LimeWire => write!(f, "LimeWire"),
            PeerClient::Lphant => write!(f, "Lphant"),
            PeerClient::Mainline => write!(f, "Mainline"),
            PeerClient::MainlineBitTorrentOrBBTor => write!(f, "Mainline BitTorrent/BBTor"),
            PeerClient::Meerkat => write!(f, "Meerkat"),
            PeerClient::Miro => write!(f, "Miro"),
            PeerClient::MonoTorrent => write!(f, "MonoTorrent"),
            PeerClient::MoonlightTorrent => write!(f, "MoonlightTorrent"),
            PeerClient::MooPolice => write!(f, "MooPolice"),
            PeerClient::NetBitTorrent => write!(f, "Net::BitTorrent"),
            PeerClient::NetTransport => write!(f, "Net Transport"),
            PeerClient::OmegaTorrent => write!(f, "OmegaTorrent"),
            PeerClient::OneSwarm => write!(f, "OneSwarm"),
            PeerClient::Pando => write!(f, "Pando"),
            PeerClient::PHPTracker => write!(f, "PHPTracker"),
            PeerClient::PicoTorrent => write!(f, "PicoTorrent"),
            PeerClient::PropagateDataClient => write!(f, "Propagate Data Client"),
            PeerClient::ProtocolBitTorrent => write!(f, "Protocol::BitTorrent"),
            PeerClient::Q4Torrent => write!(f, "Qt 4 Torrent"),
            PeerClient::QBittorrent => write!(f, "qBittorrent"),
            PeerClient::QDownload => write!(f, "QQDownload"),
            PeerClient::Retriever => write!(f, "Retriever"),
            PeerClient::RezTorrent => write!(f, "RezTorrent"),
            PeerClient::Shareaza => write!(f, "Shareaza"),
            PeerClient::ShareazaAlphaOrBeta => write!(f, "Shareaza Alpha/Beta"),
            PeerClient::SharkTorrent => write!(f, "SharkTorrent"),
            PeerClient::SoMud => write!(f, "SoMud"),
            PeerClient::SwarmScope => write!(f, "SwarmScope"),
            PeerClient::Swiftbit => write!(f, "~Swiftbit"),
            PeerClient::SymTorrent => write!(f, "SymTorrent"),
            PeerClient::TerasaurSeedBank => write!(f, "Terasaur Seed Bank"),
            PeerClient::Thunder => write!(f, "Thunder"),
            PeerClient::Torch => write!(f, "Torch"),
            PeerClient::TorrentDotNET => write!(f, "TorrentDotNET"),
            PeerClient::Torrentstorm => write!(f, "Torrentstorm"),
            PeerClient::Transmission => write!(f, "Transmission"),
            PeerClient::Tribler => write!(f, "Tribler"),
            PeerClient::TuoTu => write!(f, "TuoTu"),
            PeerClient::ULeecher => write!(f, "uLeecher!"),
            PeerClient::UTorrent => write!(f, "µTorrent"),
            PeerClient::UTorrentEmbedded => write!(f, "µTorrent Embedded"),
            PeerClient::UTorrentMac => write!(f, "µTorrent Mac"),
            PeerClient::UTorrentWeb => write!(f, "µTorrent Web"),
            PeerClient::Vagaa => write!(f, "Vagaa"),
            PeerClient::WebTorrent => write!(f, "WebTorrent"),
            PeerClient::WebTorrentDesktop => write!(f, "WebTorrent Desktop"),
            PeerClient::WeirdChineseSussyBaka => write!(f, "Weird Chinese Sussy Baka"),
            PeerClient::WeirdSussyBaka => write!(f, "Weird Sussy Baka"),
            PeerClient::XanTorrent => write!(f, "XanTorrent"),
            PeerClient::XFPlay => write!(f, "XFPlay"),
            PeerClient::XSwifter => write!(f, "XSwifter"),
            PeerClient::XTorrent => write!(f, "XTorrent"),
            PeerClient::Xunlei => write!(f, "Xunlei"),
            PeerClient::ZipTorrent => write!(f, "ZipTorrent"),
        }
    }
}

impl PeerId {
    pub fn client(&self) -> Result<PeerClient, PeerClientError> {
        if self.0[0] == b'M' {
            return Ok(PeerClient::Mainline);
        }
        if self.0[0] == b'-' {
            return PeerClient::try_from([self.0[1], self.0[2]]);
        }
        Err(PeerClientError::UnknownClient)
    }

    pub fn version(&self) -> String {
        todo!()
    }
}

pub trait Peer {
    fn id(&self) -> Option<PeerId>;
    fn addr(&self) -> SocketAddr;
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PeerStream {
    Dict(PeerDictStream),
    Bin(PeerBinStream),
}

pub type PeerDictStream = Vec<PeerDict>;
pub type PeerBinStream = Vec<PeerBin>;

#[derive(Debug, Serialize, Deserialize)]
pub struct PeerDict {
    pub id: PeerId,
    pub addr: SocketAddr,
}

/// Peer binary representation.
/// First 4 bytes are IP address, last 2 bytes are port.
/// Network byte order (big endian).
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerBin(pub [u8; 6]);

impl Peer for PeerDict {
    fn id(&self) -> Option<PeerId> {
        Some(self.id)
    }

    fn addr(&self) -> SocketAddr {
        self.addr
    }
}

impl Peer for PeerBin {
    fn id(&self) -> Option<PeerId> {
        None
    }

    fn addr(&self) -> SocketAddr {
        SocketAddr::V4(SocketAddrV4::new(
            Ipv4Addr::from(u32::from_be_bytes([
                self.0[0], self.0[1], self.0[2], self.0[3],
            ])),
            u16::from_be_bytes([self.0[4], self.0[5]]),
        ))
    }
}
