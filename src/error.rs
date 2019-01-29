use std::io;

error_chain! {

    links {
    }

    foreign_links {
        Io(io::Error);
        Liquid(liquid::Error);
        WalkDir(walkdir::Error);
        SerdeYaml(serde_yaml::Error);
        SerdeJson(serde_json::Error);
        Toml(toml::de::Error);
        Ignore(ignore::Error);
    }

    errors {
        ConfigFileMissingFields {
            description("missing fields in config file")
            display("name, description and link need to be defined in the config file to \
                    generate RSS")
        }

        UnsupportedPlatform(functionality: &'static str, platform: &'static str) {
            description("functionality is not implemented for this platform")
            display("{} is not implemented for the {} platform", functionality, platform)
        }
    }
}
