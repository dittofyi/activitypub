use activitystreams::iri_string::types::{IriStr, IriString};

pub trait IntoIri {
    fn into_iri(&self) -> IriString;
}

impl IntoIri for url::Url {
    fn into_iri(&self) -> IriString {
        self.to_string().parse().unwrap()
    }
}

pub trait IntoUrl {
    fn into_url(&self) -> url::Url;
}

impl IntoUrl for IriString {
    fn into_url(&self) -> url::Url {
        self.to_string().parse().unwrap()
    }
}

impl IntoUrl for IriStr {
    fn into_url(&self) -> url::Url {
        self.to_string().parse().unwrap()
    }
}
