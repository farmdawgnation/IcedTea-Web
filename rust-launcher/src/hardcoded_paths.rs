use os_access;
use env;
use std::string::String;
use std::fmt::Write;
use std::str::FromStr;

/*legacy variables*/
const PROGRAM_NAME: Option<&'static str> = option_env!("PROGRAM_NAME");
const LAUNCHER_BOOTCLASSPATH: Option<&'static str> = option_env!("LAUNCHER_BOOTCLASSPATH");
const SPLASH_PNG: Option<&'static str> = option_env!("SPLASH_PNG");
const JAVA: Option<&'static str> = option_env!("JAVA");
const JRE: Option<&'static str> = option_env!("JRE");
const MAIN_CLASS: Option<&'static str> = option_env!("MAIN_CLASS");
const BIN_LOCATION: Option<&'static str> = option_env!("BIN_LOCATION");
const NETX_JAR: Option<&'static str> = option_env!("NETX_JAR");
const PLUGIN_JAR: Option<&'static str> = option_env!("PLUGIN_JAR");
const JSOBJECT_JAR: Option<&'static str> = option_env!("JSOBJECT_JAR");
const TAGSOUP_JAR: Option<&'static str> = option_env!("TAGSOUP_JAR");
const RHINO_JAR: Option<&'static str> = option_env!("RHINO_JAR");
const ITW_LIBS: Option<&'static str> = option_env!("ITW_LIBS");


pub fn get_jre() -> &'static str {
    JRE.unwrap_or("JRE-dev-unspecified")
}

pub fn get_java() -> &'static str {
    JAVA.unwrap_or("JAVA-dev-unspecified")
}

pub fn get_main() -> &'static str {
    MAIN_CLASS.unwrap_or("MAIN_CLASS-dev-unspecified")
}

pub fn get_name() -> &'static str {
    PROGRAM_NAME.unwrap_or("PROGRAM_NAME-dev-unspecified")
}

pub fn get_bin() -> &'static str {
    BIN_LOCATION.unwrap_or("BIN_LOCATION-dev-unspecified")
}

pub fn get_splash() -> &'static str {
    SPLASH_PNG.unwrap_or("SPLASH_PNG-dev-unspecified")
}

pub fn get_netx() -> &'static str { NETX_JAR.unwrap_or("NETX_JAR-dev-unspecified") }

fn get_itwlibsearch() -> &'static str {
    ITW_LIBS.unwrap_or("ITW_LIBS-dev-unspecified")
}

pub fn get_bootcp() -> &'static str {LAUNCHER_BOOTCLASSPATH.unwrap_or("LAUNCHER_BOOTCLASSPATH-dev-unspecified") }

// optional deps
pub fn get_plugin() -> Option<&'static str> {
    PLUGIN_JAR
}

pub fn get_jsobject() -> Option<&'static str> { JSOBJECT_JAR }

pub fn get_tagsoup() -> Option<&'static str> { TAGSOUP_JAR }

pub fn get_rhino() -> Option<&'static str> { RHINO_JAR }


#[derive(PartialEq)]
pub enum ItwLibSearch {
    BUNDLED,
    DISTRIBUTION,
    BOTH,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseItwLibSearch { _priv: () }

impl FromStr for ItwLibSearch {

    type Err = ParseItwLibSearch;

    fn from_str(sstr: &str) -> Result<ItwLibSearch, ParseItwLibSearch> {
        if sstr == "BOTH" {
            return Ok(ItwLibSearch::BOTH);
        }
        if sstr == "BUNDLED" {
            return Ok(ItwLibSearch::BUNDLED);
        }
        if sstr == "DISTRIBUTION" {
            return Ok(ItwLibSearch::DISTRIBUTION);
        }
        return Err(ParseItwLibSearch { _priv: () })
    }
}

pub fn get_libsearch(logger: &os_access::Os) -> ItwLibSearch {
    let itw_libs_override = env::var("ITW_LIBS");
    match itw_libs_override {
        Ok(result_of_override_var) => match ItwLibSearch::from_str(&result_of_override_var) {
            Ok(result_of_override_to_enum) => {
                return result_of_override_to_enum;
            }
            _Err => {
                let mut info = String::new();
                write!(&mut info, "ITW-LIBS provided, but have invalid value of {}. Use BUNDLED, DISTRIBUTION or BOTH", result_of_override_var);
                logger.info(&info);
            }
        }
        _error => {
            //no op, continuing via get_itwlibsearch
        }
    }
    match ItwLibSearch::from_str(get_itwlibsearch()) {
        Ok(v) => {
            return v
        }
        _Err=> {
            panic!("itw-lib search out of range");
        }
    }
}


/*new variables*/

/*tests*/
#[cfg(test)]
mod tests {
    use std::str::FromStr;

    #[test]
    fn variables_non_default() {
        assert_ne!(String::from(super::get_jre()).trim(), String::from("JRE-dev-unspecified"));
        assert_ne!(String::from(super::get_java()).trim(), String::from("JAVA-dev-unspecified"));
        assert_ne!(String::from(super::get_main()).trim(), String::from("MAIN_CLASS-dev-unspecified"));
        assert_ne!(String::from(super::get_name()).trim(), String::from("PROGRAM_NAME-dev-unspecified"));
        assert_ne!(String::from(super::get_bin()).trim(), String::from("BIN_LOCATION-dev-unspecified"));
        assert_ne!(String::from(super::get_splash()).trim(), String::from("SPLASH_PNG-dev-unspecified"));
        assert_ne!(String::from(super::get_netx()).trim(), String::from("NETX_JAR-dev-unspecified"));
        assert_ne!(String::from(super::get_itwlibsearch()).trim(), String::from("ITW_LIBS-dev-unspecified"));
    }

    #[test]
    fn variables_non_empty() {
        assert_ne!(String::from(super::get_jre()).trim(), String::from(""));
        assert_ne!(String::from(super::get_java()).trim(), String::from(""));
        assert_ne!(String::from(super::get_main()).trim(), String::from(""));
        assert_ne!(String::from(super::get_name()).trim(), String::from(""));
        assert_ne!(String::from(super::get_bin()).trim(), String::from(""));
        assert_ne!(String::from(super::get_splash()).trim(), String::from(""));
        assert_ne!(String::from(super::get_netx()).trim(), String::from(""));
        assert_ne!(String::from(super::get_itwlibsearch()).trim(), String::from(""));
    }

    #[test]
    fn get_itwlibsearch_in_enumeration() {
        assert_eq!(super::get_itwlibsearch() == "BOTH" || super::get_itwlibsearch() == "BUNDLED" || super::get_itwlibsearch() == "DISTRIBUTION", true);
    }

    #[test]
    fn itw_libsearch_to_enum_test() {
        assert!(super::ItwLibSearch::from_str("BUNDLED") == Ok(super::ItwLibSearch::BUNDLED));
        assert!(super::ItwLibSearch::from_str("BOTH") == Ok(super::ItwLibSearch::BOTH));
        assert!(super::ItwLibSearch::from_str("DISTRIBUTION") == Ok(super::ItwLibSearch::DISTRIBUTION));
        assert!(super::ItwLibSearch::from_str("") == Err(super::ParseItwLibSearch { _priv: () }));
    }
}
