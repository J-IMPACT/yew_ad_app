use yew::prelude::*;
use web_sys::window;

mod adsense;
mod analytics;

use adsense::AdsenseAd;
use analytics::GoogleAnalytics;

struct Model {
    photos: Vec<String>
}

enum Msg {}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
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
            <div class="container">

                <GoogleAnalytics />

                <header>
                    <h1>{ "Yamada Taro's 富山県風景写真" }</h1>
                    <p>{ "フリー素材としてダウンロード可能です。" }</p>
                </header>

                <main>
                    <div class="gallery">
                    { for self.photos.iter().map(|photo| self.view_photo(photo)) }
                    </div>
                </main>

                <footer>
                    <AdsenseAd />
                </footer>
            </div>
        }
    }
}

impl Model {
    fn view_photo(&self, photo: &str) -> Html {
        let download_link = format!("/static/images/{}", photo);
        html! {
            <div class="photo-item">
                <img src={format!("/static/images/{}", photo)} alt={photo.to_string()} />
                <a href={download_link} download={photo.to_string()}>{"ダウンロード"}</a>
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<Model>::new().render();
}