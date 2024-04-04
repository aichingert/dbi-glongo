import gleam/io
import mungo

import collections

pub fn main() {
    let assert Ok(client) =
        mungo.start(
            "mongodb://app:app@localhost/app-db?authSource=admin",
            512,
        )

    let _ = collections.insert(client, collections.BlogUser
        (
            "mo", "mortiz", "gruber", "mo@gmail.com", "Gks3#w!kE18"
        )
    )

}
