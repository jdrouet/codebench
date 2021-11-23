use std::env;
use std::error::Error;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Environment {
    pub commit_hash: Option<String>,
    pub branch_name: Option<String>,
}

impl Environment {
    fn parse_commit_hash() -> Option<String> {
        env::var("CIRCLE_SHA1")
            .or_else(|_| env::var("TRAVIS_COMMIT"))
            .or_else(|_| env::var("CI_COMMIT_SHA"))
            .or_else(|_| env::var("GITHUB_SHA"))
            .ok()
    }

    fn parse_branch_name() -> Option<String> {
        env::var("CIRCLE_BRANCH")
            .or_else(|_| env::var("TRAVIS_BRANCH"))
            .or_else(|_| env::var("CI_COMMIT_BRANCH"))
            .ok()
    }

    /// Parses the commit hash and the branch name from the environment variables
    ///
    /// On CircleCI, the environment variable `CIRCLE_SHA1` contains the commit hash
    /// and `CIRCLE_BRANCH` contains the branch name.
    /// On Travis, `TRAVIS_COMMIT` contains the commit hash and `TRAVIS_BRANCH` contains
    /// the branch name.
    /// On GitlabCI, `CI_COMMIT_SHA` contains the commit hash and `CI_COMMIT_BRANCH`
    /// contains the branch name.
    pub fn parse() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            commit_hash: Self::parse_commit_hash(),
            branch_name: Self::parse_branch_name(),
        })
    }

    /// Set the commit hash.
    ///
    /// If `value` is `Some`, then the value will override the `commit_hash`.
    pub fn with_commit_hash(mut self, value: Option<String>) -> Self {
        if let Some(value) = value {
            self.commit_hash = Some(value);
        }
        self
    }

    /// Set the branch name.
    ///
    /// If `value` is `Some`, then the value will override the `branch_name`.
    pub fn with_branch_name(mut self, value: Option<String>) -> Self {
        if let Some(value) = value {
            self.branch_name = Some(value);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Environment;
    use env_test_util::TempEnvVar;
    use serial_test::serial;

    #[test]
    #[serial]
    fn parse_circleci() {
        let _hash = TempEnvVar::new("CIRCLE_SHA1").with("this-is-a-sha1");
        let _branch = TempEnvVar::new("CIRCLE_BRANCH").with("this-is-a-branch");
        let env = Environment::parse().unwrap();
        assert_eq!(env.commit_hash.unwrap(), "this-is-a-sha1");
        assert_eq!(env.branch_name.unwrap(), "this-is-a-branch");
    }

    #[test]
    #[serial]
    fn parse_travis() {
        let _hash = TempEnvVar::new("TRAVIS_COMMIT").with("this-is-a-sha1");
        let _branch = TempEnvVar::new("TRAVIS_BRANCH").with("this-is-a-branch");
        let env = Environment::parse().unwrap();
        assert_eq!(env.commit_hash.unwrap(), "this-is-a-sha1");
        assert_eq!(env.branch_name.unwrap(), "this-is-a-branch");
    }

    #[test]
    #[serial]
    fn parse_gitlabci() {
        let _hash = TempEnvVar::new("CI_COMMIT_SHA").with("this-is-a-sha1");
        let _branch = TempEnvVar::new("CI_COMMIT_BRANCH").with("this-is-a-branch");
        let env = Environment::parse().unwrap();
        assert_eq!(env.commit_hash.unwrap(), "this-is-a-sha1");
        assert_eq!(env.branch_name.unwrap(), "this-is-a-branch");
    }

    #[test]
    #[serial]
    fn parse_github_actions() {
        let _hash = TempEnvVar::new("GITHUB_SHA").with("this-is-a-sha1");
        let env = Environment::parse().unwrap();
        println!("env: {:?}", env);
        assert_eq!(env.commit_hash.unwrap(), "this-is-a-sha1");
        assert!(env.branch_name.is_none());
    }
}
