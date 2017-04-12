use git2::{Repository, Oid};
use semver::{Version, VersionReq};
use regex::Regex;

pub fn get_latest_commit(repo: &Repository) -> Option<String> {
    match repo.head() {
        Ok(r) => match r.resolve() {
            Ok(ref r) => match r.target() {
                Some(id) => Some(format!("{}", id)),
                None => None,
            },
            _ => None,
        },
        _ => None,
    }
}

pub fn get_latest_version(current_version: String, repo: &Repository) -> String {
    let mut version = current_version;
    match VersionReq::parse(version.as_str()) {
        Ok(version_rule) => {
            match repo.tag_names(None) {
                Ok(tag_names) => {
                    let mut selected_tag = None;
                    let re = Regex::new(r"^v?([0-9]+)[.]?([0-9]*)[.]?([0-9]*)([-]?.*)").unwrap();
                    for t in tag_names.iter() {
                        let tag_name = match t {
                            Some(name) => name,
                            None => continue,
                        };
                        let tag_version_str = match re.captures(t.unwrap()) {
                            Some(caps) => format!("{}.{}.{}{}",
                                                  match caps.get(1) {
                                                      Some(c) => {
                                                          let n = c.as_str();
                                                          if n.is_empty() {
                                                              continue
                                                          } else {
                                                              n
                                                          }
                                                      },
                                                      _ => continue,
                                                  },
                                                  match caps.get(2) {
                                                      Some(c) => {
                                                          let n = c.as_str();
                                                          if n.is_empty() {
                                                              "0"
                                                          } else {
                                                              n
                                                          }
                                                      },
                                                      _ => "0",
                                                  },
                                                  match caps.get(3) {
                                                      Some(c) => {
                                                          let n = c.as_str();
                                                          if n.is_empty() {
                                                              "0"
                                                          } else {
                                                              n
                                                          }
                                                      },
                                                      _ => "0",
                                                  },
                                                  match caps.get(4) {
                                                      Some(c) => c.as_str(),
                                                      _ => "",
                                                  }),
                            None => continue,
                        };
                        let tag_version = match Version::parse(tag_version_str.as_str()) {
                            Ok(ver) => ver,
                            _ => continue,
                        };
                        if version_rule.matches(&tag_version) && (selected_tag.is_none() || tag_version > selected_tag.clone().unwrap()) {
                            selected_tag = Some(tag_version);
                            version = tag_name.to_owned();
                        }
                    }
                },
                _ => (),
            };
        },
        _ => (),
    }
    version
}
