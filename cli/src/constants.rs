const REPO_OWNER: &str = "Emii-lia";
const REPO_NAME: &str = "yewi-kit";
const REPO_BRANCH: &str = "master";
const RAW_GITHUB_URL: &str = "https://raw.githubusercontent.com";

pub(crate) fn get_repo_config() -> (String, String, String, String) {
    (
        REPO_OWNER.to_string(),
        REPO_NAME.to_string(),
        REPO_BRANCH.to_string(),
        RAW_GITHUB_URL.to_string(),
    )
}