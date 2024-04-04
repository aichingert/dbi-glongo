import gleam/io
import mungo
import mungo/aggregation.{
    match
}
import bison/bson

pub fn main() {
    let assert Ok(client) =
        mungo.start(
            "mongodb://app:app@localhost/app-db?authSource=admin",
            512,
        )

    let blog_users =
        client
        |> mungo.collection("blogUsers")

    let _ = 
        blog_users
        |> mungo.insert_many([[#("username", bson.String("jmorrow"))]], 128)

    let assert Ok(lol) =
        blog_users
        |> match([#(

    let users = mungo.to_list(lol, 128)
    io.debug(users)
}
