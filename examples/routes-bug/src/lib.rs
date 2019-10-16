#[macro_use]
extern crate seed;

use seed::prelude::*;

#[derive(Default)]
struct Model {
    count: u32,
    url: Option<Url>,
}

#[derive(Debug, Clone)]
enum Msg {
    Route(Url),
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::Route(url) => {
            model.url = Some(url);
        }
    }
    model.count += 1;
}

fn view(model: &Model) -> impl View<Msg> {
    div![
        if let Some(url) = &model.url {
            p![
                if let Some(ref h) = url.hash {
                    format!("Current hash: '{}'", h)
                } else {
                    "There is no hash in the url!".into()
                },
                br![],
                format!("URL: {:?}", url)
            ]
        } else {
            p!["Currently the URL is unknown"]
        },
        p![format!("{} route events were fired", model.count)]
    ]
}

fn routes(url: Url) -> Msg {
    Msg::Route(url)
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(|_, _| Model::default(), update, view)
        .routes(routes)
        .finish()
        .run();
}
