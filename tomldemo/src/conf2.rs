
enum Profiles {
    DEV(Profile),
    STAG(Profile),
    PROD(Profile),
}

struct Profile {
    name: String,
    profile: BaseProfile,
    db: DBConfig,
}

struct BaseProfile {
    address: String,
    port: String,
    workers: u8
}

struct DBConfig {
    adapter: String,
    dbName: String,
    pool: u8,
}