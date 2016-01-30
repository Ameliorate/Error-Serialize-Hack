/// Used if an error from a popular crate is better being fully seralized, or if it is in std and has fields that should be preserved.
///
/// This enum may be expanded in the future. For this reason, you should not exaustively match aganst it.
#[derive(Serialize, Deserialize, Debug)]
pub enum RealError {
    /// This ensures that you cannot match the whole enum, and must always consiter for more feilds in the furure.
    #[doc(hidden)]
    __Nonexhaustive,
}
