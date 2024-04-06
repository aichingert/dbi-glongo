import gleam/erlang
import gleam/erlang/process.{type Selector, type Subject}
import gleam/json
import gleam/result
import gleam/otp/actor
import gleam/bytes_builder
import gleam/option.{type Option, None}
import gleam/http/request.{type Request}
import gleam/http/response.{type Response}
import lustre
import lustre/attribute
import lustre/element
import lustre/element/html.{html}
import lustre/server_component
import mist.{
  type Connection, type ResponseData, type WebsocketConnection,
  type WebsocketMessage,
}

import blog

pub fn main() {
  let assert Ok(_) = fn(req: Request(Connection)) -> Response(ResponseData) {
    case request.path_segments(req) {
      ["app"] ->
        mist.websocket(
          request: req,
          on_init: socket_init,
          on_close: socket_close,
          handler: socket_update,
        )
      ["lustre-server-component.mjs"] -> {
        let assert Ok(priv) = erlang.priv_directory("lustre")
        let path = priv <> "/static/lustre-server-component.mjs"

        mist.send_file(path, offset: 0, limit: None)
        |> result.map(fn(script) {
          response.new(200)
          |> response.prepend_header("content-type", "application/javascript")
          |> response.set_body(script)
        })
        |> result.lazy_unwrap(fn() {
          response.new(404)
          |> response.set_body(mist.Bytes(bytes_builder.new()))
        })
      }
      _ -> 
        response.new(200)
        |> response.prepend_header("content-type", "text/html")
        |> response.set_body(
            html([], [
                html.head([], [
                    html.link([
                        attribute.rel("stylesheet"),
                        attribute.href(
                            "https://cdn.jsdelivr.net/gh/lustre-labs/ui/priv/styles.css",
                        ),
                    ]),
                    html.script(
                        [
                            attribute.type_("module"),
                            attribute.src("/lustre-server-component.mjs"),
                        ],
                        "",
                    ),
                ]),
                html.body([], [
                    server_component.component([server_component.route("/app")]),
                ]),
            ])
            |> element.to_document_string_builder
            |> bytes_builder.from_string_builder
            |> mist.Bytes
        ) 
    } 
  }
  |> mist.new
  |> mist.port(3000)
  |> mist.start_http

  process.sleep_forever()
}

type Blog =
  Subject(lustre.Action(blog.Msg, lustre.ServerComponent))

fn socket_init(
  conn: WebsocketConnection,
) -> #(Blog, Option(Selector(lustre.Patch(blog.Msg)))) {
  let app = blog.app()
  let assert Ok(blog) = lustre.start_actor(app, 0)

  process.send(
    blog,
    server_component.subscribe("ws", fn(patch) {
      let _ = patch
      |> server_component.encode_patch
      |> json.to_string
      |> mist.send_text_frame(conn, _)

      Nil
    }),
  )

  #(blog, None)
}

fn socket_update(
  blog: Blog,
  _conn: WebsocketConnection,
  msg: WebsocketMessage(lustre.Patch(blog.Msg)),
) {
  case msg {
    mist.Text(json) -> {
      case json.decode(json, server_component.decode_action) {
        Ok(action) -> process.send(blog, action)
        Error(_) -> Nil
      }

      actor.continue(blog)
    }
    mist.Binary(_) -> actor.continue(blog)
    mist.Custom(_) -> actor.continue(blog)
    mist.Closed | mist.Shutdown -> actor.Stop(process.Normal)
  }
}

fn socket_close(blog: Blog) {
  process.send(blog, lustre.shutdown())
}
