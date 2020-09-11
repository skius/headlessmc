use crate::connection::packets::types::*;
use protocol::{Parcel, Settings, hint::Hints};
use std::io::{Write, Read};



#[derive(Protocol, Clone, Debug, PartialEq)]
#[protocol(discriminant = "integer")]
#[repr(u8)] // Force discriminators to be 8-bit.
pub enum Data {
    // #[protocol(discriminator(0x21))]
    // KeepAlive(KeepAlive),
    #[protocol(discriminator(0x00))]
    SpawnEntity,
    SpawnExperienceOrb,
    SpawnWeatherEntity,
    SpawnLivingEntity,
    SpawnPainting,
    SpawnPlayer,
    EntityAnimation,
    Statistics,
    AcknowledgePlayerDigging,
    BlockBreakAnimation,
    BlockEntityData,
    BlockAction,
    BlockChange,
    BossBar,
    ServerDifficulty,
    ChatMessage(ChatMessage),
    MultiBlockChange,
    TabComplete,
    DeclareCommands,
    WindowConfirmation,
    CloseWindow,
    WindowItems,
    WindowProperty,
    SetSlot,
    SetCooldown,
    PluginMessage,
    NamedSoundEffect,
    Disconnect(Disconnect),
    EntityStatus,
    Explosion,
    UnloadChunk,
    ChangeGameState,
    OpenHorseWindow,
    KeepAlive(KeepAlive),
    ChunkData,
    Effect,
    Particle,
    UpdateLight,
    JoinGame,
    MapData,
    TradeList,
    EntityPosition,
    EntityPositionandRotation,
    EntityRotation,
    EntityMovement,
    VehicleMove,
    OpenBook,
    OpenWindow,
    OpenSignEditor,
    CraftRecipeResponse,
    PlayerAbilities,
    CombatEvent,
    PlayerInfo,
    FacePlayer,
    PlayerPositionAndLook,
    UnlockRecipes,
    DestroyEntities,
    RemoveEntityEffect,
    ResourcePackSend,
    Respawn,
    EntityHeadLook,
    SelectAdvancementTab,
    WorldBorder,
    Camera,
    HeldItemChange,
    UpdateViewPosition,
    UpdateViewDistance,
    DisplayScoreboard,
    EntityMetadata,
    AttachEntity,
    EntityVelocity,
    EntityEquipment,
    SetExperience,
    UpdateHealth,
    ScoreboardObjective,
    SetPassengers,
    Teams,
    UpdateScore,
    SpawnPosition,
    TimeUpdate,
    Title,
    EntitySoundEffect,
    SoundEffect,
    StopSound,
    PlayerListHeaderAndFooter,
    NBTQueryResponse,
    CollectItem,
    EntityTeleport,
    Advancements,
    EntityProperties,
    EntityEffect,
    DeclareRecipes,
    Tags,
    // Ignore(Ignore),
}

// Manually handling clientbound play Data, because I ignore all non-keepalive packets
// impl Parcel for Data {
//     const TYPE_NAME: &'static str = "Data";
//
//     fn read_field(mut read: &mut dyn Read, settings: &Settings, mut hints: &mut Hints) -> Result<Self, protocol::Error> {
//         let packet_id = VarInt::read_field(&mut read, &settings, &mut hints).unwrap();
//         if packet_id.val == 0x21 {
//             // KeepAlive
//             let keep_alive = KeepAlive::read_field(&mut read, &settings, &mut hints)?;
//             Ok(Data::KeepAlive(keep_alive))
//         } else {
//             // Ignore
//             Ok(Data::Ignore(Ignore))
//         }
//     }
//
//     fn write_field(&self, write: &mut dyn Write, settings: &Settings, hints: &mut Hints) -> Result<(), protocol::Error> {
//         unimplemented!()
//     }
// }

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct KeepAlive {
    pub keep_alive_id: i64,
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct ChatMessage {
    pub json_data: Chat,
    pub position: u8,
}

#[derive(Protocol, Clone, Debug, PartialEq)]
pub struct Disconnect {
    pub reason: Chat,
}

// #[derive(Clone, Debug, PartialEq)]
// pub struct Ignore;