pub mod methods {
    pub mod domain_fetch;
    pub mod domain_update;
    pub mod driver;
    pub mod ip;
    pub mod zone_get;
    pub mod test;
}

pub mod structs {
    pub mod cloudflare {

        pub mod domain;
        pub mod result;
        pub mod zone_get;

    }
    pub mod ip;
    pub mod mutex;
}