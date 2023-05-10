use anyhow::Result;
use itertools::Itertools;
use octorust::{
    auth::Credentials,
    types::{Order, ReposListOrgSort, ReposListOrgType, ReposListUserType},
    Client,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let gh = Client::new(
        String::from("list-all-my-repos"),
        Credentials::Token(String::from(
            env::var("GITHUB_TOKEN").expect("You must set the GITHUB_TOKEN env var"),
        )),
    )?;
    for repo in gh
        .repos()
        .list_all_for_user(
            "autarch",
            ReposListUserType::All,
            ReposListOrgSort::FullName,
            Order::Asc,
        )
        .await?
        .into_iter()
        .chain(
            gh.repos()
                .list_all_for_org(
                    "houseabsolute",
                    ReposListOrgType::All,
                    ReposListOrgSort::FullName,
                    Order::Asc,
                )
                .await?,
        )
        .unique_by(|r| r.html_url.clone())
        .sorted_by(|a, b| Ord::cmp(&a.full_name.to_lowercase(), &b.full_name.to_lowercase()))
    {
        println!(
            "{}\t{}\t{}",
            repo.full_name,
            repo.created_at.unwrap().date().format("%Y-%m-%d"),
            repo.html_url,
        );
    }

    Ok(())
}
