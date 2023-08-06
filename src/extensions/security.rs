use activitystreams::{
    actor::{ApActor, AsApActor},
    base::{AsBase, Base, Extends},
    iri_string::types::IriString,
    markers,
    object::{AsObject, Object},
    unparsed::*,
};

/// First, we'll define our public key types

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKeyValues {
    pub id: IriString,
    pub owner: IriString,
    pub public_key_pem: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey<Inner> {
    pub public_key: PublicKeyValues,

    #[serde(flatten)]
    pub inner: Inner,
}

/// Then, we'll implement Extends so we can produce a PublicKey<Object> from an AnyBase.

impl<Inner> Extends for PublicKey<Inner>
where
    Inner: Extends<Error = serde_json::Error> + UnparsedMut,
{
    type Kind = Inner::Kind;
    type Error = serde_json::Error;

    fn extends(base: Base<Self::Kind>) -> Result<Self, Self::Error> {
        let mut inner = Inner::extends(base)?;

        Ok(PublicKey {
            public_key: inner.unparsed_mut().remove("publicKey")?,
            inner,
        })
    }

    fn retracts(self) -> Result<Base<Self::Kind>, Self::Error> {
        let PublicKey {
            public_key,
            mut inner,
        } = self;

        inner.unparsed_mut().insert("publicKey", public_key)?;

        inner.retracts()
    }
}

/// Auto-implement Base, Object, and Actor when Inner supports it
impl<Inner> markers::Base for PublicKey<Inner> where Inner: markers::Base {}
impl<Inner> markers::Object for PublicKey<Inner> where Inner: markers::Object {}
impl<Inner> markers::Actor for PublicKey<Inner> where Inner: markers::Actor {}

/// If we want to easily access getters and setters for internal types, we'll need to forward
/// those, too.

/// Forward for base methods
///
/// This allows us to access methods related to `context`, `id`, `kind`, `name`,
/// `media_type`, and `preview` directly from the PublicKey struct
impl<Inner> AsBase for PublicKey<Inner>
where
    Inner: AsBase,
{
    type Kind = Inner::Kind;

    fn base_ref(&self) -> &Base<Self::Kind> {
        self.inner.base_ref()
    }

    fn base_mut(&mut self) -> &mut Base<Self::Kind> {
        self.inner.base_mut()
    }
}

/// Forward for object methods
///
/// This allows us to access methods related to `url`, `generator`, `start_time`, `duration`,
/// and more directly from the PublicKey struct
impl<Inner> AsObject for PublicKey<Inner>
where
    Inner: AsObject,
{
    type Kind = Inner::Kind;

    fn object_ref(&self) -> &Object<Self::Kind> {
        self.inner.object_ref()
    }

    fn object_mut(&mut self) -> &mut Object<Self::Kind> {
        self.inner.object_mut()
    }
}

/// Forward for ActivityPub actor methods
///
/// This allows us to access methods related to `inbox`, `outbox`, `following`, `followers`,
/// `liked`, `streams`, `endpoints`, and more directly from the PublicKey struct
impl<Inner> AsApActor for PublicKey<Inner>
where
    Inner: AsApActor,
{
    type Inner = Inner::Inner;

    fn ap_actor_ref(&self) -> &ApActor<Self::Inner> {
        self.inner.ap_actor_ref()
    }

    fn ap_actor_mut(&mut self) -> &mut ApActor<Self::Inner> {
        self.inner.ap_actor_mut()
    }
}

/// If we want to be able to extend from our own type, we'll need to forward some
/// implementations, and create some traits

/// Make it easy for downstreams to get an Unparsed
impl<Inner> UnparsedMut for PublicKey<Inner>
where
    Inner: UnparsedMut,
{
    fn unparsed_mut(&mut self) -> &mut Unparsed {
        self.inner.unparsed_mut()
    }
}

/// Create our own extensible trait
pub trait AsPublicKey<Inner> {
    fn public_key_ref(&self) -> &PublicKey<Inner>;
    fn public_key_mut(&mut self) -> &mut PublicKey<Inner>;
}

/// Implement it
impl<Inner> AsPublicKey<Inner> for PublicKey<Inner> {
    fn public_key_ref(&self) -> &Self {
        self
    }

    fn public_key_mut(&mut self) -> &mut Self {
        self
    }
}

/// And now create helper methods
pub trait PublicKeyExt<Inner>: AsPublicKey<Inner> {
    /// Borrow the public key's ID
    fn key_id<'a>(&'a self) -> &'a IriString
    where
        Inner: 'a,
    {
        &self.public_key_ref().public_key.id
    }

    /// Set the public key's ID
    fn set_key_id(&mut self, id: IriString) -> &mut Self {
        self.public_key_mut().public_key.id = id;
        self
    }

    /// Borrow the public key's Owner
    fn key_owner<'a>(&'a self) -> &'a IriString
    where
        Inner: 'a,
    {
        &self.public_key_ref().public_key.owner
    }

    /// Set the public key's Owner
    fn set_key_owner(&mut self, owner: IriString) -> &mut Self {
        self.public_key_mut().public_key.owner = owner;
        self
    }

    /// Borrow the public key's PEM encoded value
    fn key_pem<'a>(&'a self) -> &'a str
    where
        Inner: 'a,
    {
        &self.public_key_ref().public_key.public_key_pem
    }

    /// Set the public key's PEM encoded value
    ///
    /// In a real application, this might take a different type, such as RSA's RSAPublicKey, or
    /// OpenSSL's or Ring's version
    fn set_key_pem<T>(&mut self, pem: T) -> &mut Self
    where
        T: Into<String>,
    {
        self.public_key_mut().public_key.public_key_pem = pem.into();
        self
    }
}

/// Finally, we'll automatically implement PublicKeyExt for any type implementing AsPublicKey
impl<T, Inner> PublicKeyExt<Inner> for T where T: AsPublicKey<Inner> {}
