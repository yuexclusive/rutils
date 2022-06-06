use crate::config;
use futures::executor::block_on;
use meilisearch_sdk::client::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Movie {
    id: usize,
    title: String,
    genres: Vec<String>,
}

pub fn client() -> Client {
    let client = Client::new(
        &config::CONFIG.meilisearch.address,
        &config::CONFIG.meilisearch.api_key,
    );
    client
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_search() {
        let index = client().index("movies2");

        let res = index
            .search()
            .with_query("Woman")
            .execute::<Movie>()
            .await
            .unwrap();

        println!("{:?}", res)
    }

    #[tokio::test]
    async fn search_index() {
        let client = client();

        // An index is where the documents are stored.

        // client.index("movies2").delete().await.unwrap();

        // Add some movies in the index. If the index 'movies' does not exist, Meilisearch creates it when you first add the documents.
        client
            .index("movies2")
            .add_documents(
                &[
                    Movie {
                        id: 1,
                        title: String::from("Carol1"),
                        genres: vec!["Romance".to_string(), "Drama".to_string()],
                    },
                    Movie {
                        id: 2,
                        title: String::from("Wonder Woman"),
                        genres: vec!["Action".to_string(), "Adventure".to_string()],
                    },
                    Movie {
                        id: 3,
                        title: String::from("Life of Pi"),
                        genres: vec!["Adventure".to_string(), "Drama".to_string()],
                    },
                    Movie {
                        id: 4,
                        title: String::from("Mad Max"),
                        genres: vec!["Adventure".to_string(), "Science Fiction".to_string()],
                    },
                    Movie {
                        id: 5,
                        title: String::from("Moana"),
                        genres: vec!["Fantasy".to_string(), "Action".to_string()],
                    },
                    Movie {
                        id: 6,
                        title: String::from("Philadelphia"),
                        genres: vec!["Drama".to_string()],
                    },
                ],
                Some("id"),
            )
            .await
            .unwrap();
    }
}
