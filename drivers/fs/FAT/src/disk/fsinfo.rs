#[repr(C, packed)]
pub struct FSInfo {
    /// Lead signature.
    lead_signature: u32,
    #[doc(hidden)]
    reserved0: [u8; 480],
    /// Middle signature.
    mid_signature: u32,
    /// The last known free cluster count on the volume. This should be checked; accuracy cannot be guaranteed.
    last_known_free_cluster_count: u32,
    /// The cluster number at which the file system should start looking for available clusters.
    available_cluster_hint: u32,
    #[doc(hidden)]
    reserved1: [u8; 12],
    /// Trailing signature.
    trail_signature: u32
}

#[allow(unused)]
impl FSInfo {
    pub const LEAD_FSINFO_SIGNATURE: u32 = 0x41615252;
    pub const MID_FSINFO_SIGNATURE: u32 = 0x61417272;
    pub const TRAIL_FSINFO_SIGNATURE: u32 = 0xAA550000;
}