#[cynic::schema_for_derives(file = r#"schema.graphql"#, module = "schema")]
mod queries {
    use super::schema;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Query")]
    pub struct GetSponsors {
        pub viewer: User,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct User {
        #[arguments(last = 10)]
        pub sponsorships_as_maintainer: SponsorshipConnection,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct SponsorshipConnection {
        pub nodes: Option<Vec<Option<Sponsorship>>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct Sponsorship {
        pub privacy_level: SponsorshipPrivacy,
        pub tier: Option<SponsorsTier>,
        pub sponsor: Option<User2>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "User")]
    pub struct User2 {
        pub email: String,
        pub login: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    pub struct SponsorsTier {
        pub closest_lesser_value_tier: Option<SponsorsTier2>,
        pub is_custom_amount: bool,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "SponsorsTier")]
    pub struct SponsorsTier2 {
        pub monthly_price_in_dollars: i32,
        pub is_one_time: bool,
    }

    #[derive(cynic::Enum, Clone, Copy, Debug)]
    pub enum SponsorshipPrivacy {
        Private,
        Public,
    }
}

mod schema {
    cynic::use_schema!(r#"schema.graphql"#);
}
