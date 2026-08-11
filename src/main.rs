enum Implementation {
    BTree,
    LSMTree,
}

struct DatabaseOptions {
    name: String,
    implementation: Implementation,
}

struct DatabaseProperties {
    file_path: String,
}

struct Database<T: DatabaseTable<R>, R> {
    tables: Vec<T>,
}

type PrimaryKey = u64;

pub trait DatabaseTable<R> {
    fn insert(&self, row: R);
    fn update(&self, key: PrimaryKey, row: R);
    fn get(&self, key: PrimaryKey) -> Option<R>;
    fn delete(&self, key: PrimaryKey) -> Option<R>;
}

struct LSMTreeDatabaseTable {}

impl<R> DatabaseTable<R> for LSMTreeDatabaseTable {
    fn insert(&self, row: R) {}

    fn update(&self, key: PrimaryKey, row: R) {}

    fn get(&self, key: PrimaryKey) -> Option<R> {
        return None;
    }

    fn delete(&self, key: PrimaryKey) -> Option<R> {
        return None;
    }
}

fn main() {
    println!("Hello, world!");
}
