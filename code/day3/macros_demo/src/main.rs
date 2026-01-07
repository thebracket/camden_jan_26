use std::collections::HashMap;

macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $value); )*
        map
    }};
}

fn main() {
    let ports = hashmap! {
        "http" => 80,
        "https" => 443,
    };

    println!("{ports:?}");
}

