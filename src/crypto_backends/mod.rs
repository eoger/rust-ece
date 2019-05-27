#[cfg(feature = "backend-openssl")]
pub use self::openssl::{
    OpenSSLCrypto as CryptoImpl, OpenSSLLocalKeyPair as LocalKeyPairImpl,
    OpenSSLRemotePublicKey as RemotePublicKeyImpl,
};

#[cfg(feature = "backend-openssl")]
pub mod openssl;
