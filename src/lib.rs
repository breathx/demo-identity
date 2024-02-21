// TODO (core): consider ability to query your own code hash.
// TODO (core): impl Display to `gstd` id primitives.

#![no_std]

use gstd::{
    exec, format,
    msg::{self, reply},
    ActorId, String, Vec,
};
use parity_scale_codec::{Decode, Encode};

/// Admin and the user to be described by the identity program.
static mut USER: ActorId = ActorId::zero();

/// Data defining the identity.
static mut DATA: IdentityData = IdentityData::dummy();

/// The init functions purpose is to store the persona-s id.
#[no_mangle]
extern "C" fn init() {
    // Storing the origin user.
    unsafe { USER = msg::source() };

    // Storing default identity data.
    unsafe { DATA = IdentityData::new(&USER) };
}

/// The handle functions purpose is to provide query opportunity for everybody
/// and modifications abilities for the identified user.
#[no_mangle]
extern "C" fn handle() {
    // Decoding requested command.
    let command = msg::load().expect("Failed to decode `Command`");

    // Processing command.
    match command {
        Command::Get => {
            reply(unsafe { &DATA }, 0).expect("Failed to share the data");
        }
        Command::Update(modifications) => {
            for modification in modifications {
                modification.apply(unsafe { &mut DATA });
            }
        }
    }
}

/// Identity data.
#[derive(Debug, Encode)]
pub struct IdentityData {
    /// Name or nickname.
    pub name: String,
    /// Socials link or any additional data.
    pub socials: String,
    /// Keywords of interests.
    pub keywords: Vec<String>,
    /// Target region.
    pub region: Region,
}

impl IdentityData {
    /// Dummy function for initial setting.
    pub const fn dummy() -> Self {
        Self {
            name: String::new(),
            socials: String::new(),
            keywords: Vec::new(),
            region: Region::Earth,
        }
    }

    /// Creates new identity data.
    ///
    /// NOTE: Never call out of executor context.
    pub fn new(user: &'static ActorId) -> Self {
        let name = format!("0x{}", hex::encode(user.as_ref()));
        let socials = format!("vara.go/0x{}", hex::encode(exec::program_id().as_ref()));

        IdentityData {
            name,
            socials,
            keywords: Vec::with_capacity(32),
            region: Region::Earth,
        }
    }
}

/// The command for handle processing.
#[derive(Debug, Decode)]
pub enum Command {
    /// Returns all associated with identity data.
    Get,
    /// Applies given modifications to identity data.
    Update(Vec<Modification>),
}

/// Modification that could be applied.
#[derive(Debug, Decode)]
pub enum Modification {
    /// Name.
    Name(String),
    /// Socials link or any other web2 resource.
    Socials(String),
    /// Keywords/tags of your interests.
    Keywords(Vec<String>),
    /// Region the identity is living in.
    /// Used to offer personalized advertisement deals.
    Region(Region),
}

impl Modification {
    /// Applies this modification to the identity data.
    pub fn apply(self, identity: &mut IdentityData) {
        match self {
            Self::Name(name) => identity.name = name,
            Self::Socials(socials) => identity.socials = socials,
            Self::Keywords(keywords) => identity.keywords = keywords,
            Self::Region(region) => identity.region = region,
        }
    }
}

/// Region specification.
#[derive(Debug, Encode, Decode)]
pub enum Region {
    /// Unspecified region.
    Earth,
    /// Europe.
    Europe,
    /// Latin America.
    LatAm,
}
