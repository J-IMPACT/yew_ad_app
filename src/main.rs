use yew::prelude::*;
use web_sys::window;

mod adsense;
mod analytics;

struct Model {
    photos: Vec<String>
}

enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        gtag_script();
        adsense_script();

        let photos = vec![
            "img1.jpg".to_string(),
            "img2.jpg".to_string(),
            "img3.jpg".to_string()
        ];

        Self { photos }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {

        }
    }
}

impl Model {
    fn view_photo(&self, photo: &str) -> Html {
        let download_link = format!("/static/images/{}", photo);
        html! {
            <div class="photo-item">
                <img src={format!("/static/images/{}", photo)} alt={photo} />
                <a href={download_link} download={photo}>{"ダウンロード"}</a>
            </div>
        }
    }
}

fn gtag_script() {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let script = document.create_element("script").unwrap();
    script.set_attribute("src", "https://www.googletagmanager.com/gtag/js?id=GA_TRACKING_ID").unwrap();
    document.head().unwrap().append_child(&script).unwrap();

    let script = document.create_element("script").unwrap();
    script.set_text_content(Some(
        r#"
        window.dataLayer = window.dataLayer || [];
        function gtag(){dataLayer.push(arguments);}
        gtag('js', new Date());
        gtag('config', 'GA_TRACKING_ID');
        "#,
    ));
    document.head().unwrap().append_child(&script).unwrap();
}

fn adsense_script() {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let script = document.create_element("script").unwrap();
    script.set_text_content(Some(
        r#"
        (adsbygoogle = window.adsbygoogle || []).push({
            google_ad_client: "ADSENSE_CLIENT_ID",
            enable_page_level_ads: true
        });
        "#,
    ));
    document.head().unwrap().append_child(&script).unwrap();
}

#[function_component(AdSenseAd)]
fn adsense_ad() -> Html {
    html! {
        <div id="adsense-container">
            {"AdSense広告がここに表示されます。"}
        </div>
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}