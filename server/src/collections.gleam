import mungo
import bison/bson

pub type Entity {
    BlogUser(username: String, first_name: String, last_name: String, email: String, password: String)
    BlogEntry(titel: String)
    BlogCategories(category: String)
    Comments(comment: String)
}

pub fn insert(client, entity: Entity) {
    case entity {
        BlogUser(u, f, l, e, p) -> {
            let _ = 
                client
                |> mungo.collection("blogUsers")
                |> mungo.insert_one([
                    #("username", bson.String(u)),
                    #("first_name", bson.String(f)),
                    #("last_name", bson.String(l)),
                    #("email", bson.String(e)),
                    #("password", bson.String(p))
                ], 128)
        }
        BlogEntry(t) -> {
            let _ = 
                client
                |> mungo.collection("blogEntry")
                |> mungo.insert_one([#("titel", bson.String(t))], 128)
        }
        BlogCategories(c) -> {
            let _ = 
                client
                |> mungo.collection("blogCategories")
                |> mungo.insert_one([#("category", bson.String(c))], 128)
        }
        Comments(co) -> {
            let _ = 
                client
                |> mungo.collection("Comments")
                |> mungo.insert_one([#("comment", bson.String(co))], 128)
        }
    }
}

pub fn add(a: Int) {
    a + 1
}
