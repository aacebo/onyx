mod architecture;

pub use architecture::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "lowercase")]
pub enum Device {
    Cpu,
    Cuda(usize),
    Metal(usize),
}

impl Device {
    pub const fn id(self) -> Option<usize> {
        match self {
            Self::Cuda(v) => Some(v),
            Self::Metal(v) => Some(v),
            _ => None,
        }
    }

    pub const fn architecture(self) -> DeviceArchitecture {
        match self {
            Self::Cpu => DeviceArchitecture::Cpu,
            Self::Cuda(_) => DeviceArchitecture::Cuda,
            Self::Metal(_) => DeviceArchitecture::Metal,
        }
    }
}
