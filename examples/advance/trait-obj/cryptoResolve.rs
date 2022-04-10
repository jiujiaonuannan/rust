/// An object that resolves the providers of Noise crypto choices
pub trait CryptoResolver {
	/// Provide an implementation of the Random trait or None if none available.
	fn resolve_rng(&self) -> Option<Box<dyn Random>>;

	/// Provide an implementation of the Dh trait for the given DHChoice or None if unavailable.
	fn resolve_dh(&self, choice: &DHChoice) -> Option<Box<dyn Dh>>;

	/// Provide an implementation of the Hash trait for the given HashChoice or None if unavailable.
	fn resolve_hash(&self, choice: &HashChoice) -> Option<Box<dyn Hash>>;

	/// Provide an implementation of the Cipher trait for the given CipherChoice or None if unavailable.
	fn resolve_cipher(&self, choice: &CipherChoice) -> Option<Box<dyn Cipher>>;

	/// Provide an implementation of the Kem trait for the given KemChoice or None if unavailable
	#[cfg(feature = "hfs")]
	fn resolve_kem(&self, _choice: &KemChoice) -> Option<Box<dyn Kem>> {
			None
	}
}