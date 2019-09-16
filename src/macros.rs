#[macro_export]
macro_rules! domain {
    ($domain:tt) => {
        libsip::Domain::Domain($domain.into(), None)
    };
    ($domain:tt, $port:tt) => {
        libsip::Domain::Domain($domain.into(), Some($port))
    };
}

#[macro_export]
macro_rules! ip_domain {
    ($a:tt, $b:tt, $c:tt, $d:tt) => {
        libsip::Domain::Ipv4(::std::net::Ipv4Addr::new($a, $b, $c, $d), None)
    };

    ($a:tt, $b:tt, $c:tt, $d:tt, $port:tt) => {
        libsip::Domain::Ipv4(::std::net::Ipv4Addr::new($a, $b, $c, $d), Some($port))
    };
}

#[macro_export]
macro_rules! uri_auth {
    ($u: tt) => {
        libsip::uri::UriAuth::new($u)
    };
    ($u: tt, $p: tt) => {
        libsip::uri::UriAuth::new($u).password($p)
    }
}
