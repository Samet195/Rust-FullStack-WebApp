// // Extenal Crates Imports
// use anyhow::Result;
// use common::types::Repository;
// use common::Config;
// use gloo_net::http::Request;

// pub async fn get_repos() -> Result<Vec<Repository>> {
//     let Config { uname, .. } = Default::default();

//     let mut data = Request::get(&format!("https://api.github.com/users/{}/repos", uname))
//         .send()
//         .await?
//         .json::<Vec<Repository>>()
//         .await?;

//     data.sort_by(|x, y| x.updated_at.cmp(&y.updated_at));
//     data.reverse();

//     Ok(data)
// }

// pub async fn get_repo(repo: String) -> Result<Repository> {
//     let Config { uname, .. } = Default::default();

//     Ok(
//         Request::get(&format!("https://api.github.com/repos/{}/{}", uname, repo))
//             .send()
//             .await?
//             .json::<Repository>()
//             .await?,
//     )
// }
