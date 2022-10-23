use crate::database::clients::ClientFilter;
use crate::ClientDB;

#[test]
fn imdb_add_client() {
    let mut imdb = crate::database::in_memory_database::InMemDB::new();

    imdb.add_client("larry", "lolasd").unwrap();

    print!("{:?}", imdb);
}

#[test]
fn imdb_update_client() {
    let mut imdb = crate::database::in_memory_database::InMemDB::new();

    imdb.add_client("larry", "lolasd").unwrap();

    let mut c = imdb
        .get_clients(ClientFilter::new().with_name("larry".to_string()))
        .unwrap()
        .pop()
        .unwrap();
    c.name = "astracalustro".to_string();
    imdb.update_client(&c).unwrap();

    println!("{:?}", imdb);
}
