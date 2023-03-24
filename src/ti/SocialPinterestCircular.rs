#[cfg(feature = "TiSocialPinterestCircular")]
use leptos::*;
#[cfg(feature = "TiSocialPinterestCircular")]
///This icon requires the feature `TiSocialPinterestCircular` to be enabled.
#[component]
pub fn SocialPinterestCircular(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < g xmlns = "http://www.w3.org/2000/svg" >< path d =
        "M12 21c-4.963 0-9-4.037-9-9s4.037-9 9-9 9 4.037 9 9-4.037 9-9 9zm0-16c-3.859 0-7 3.141-7 7s3.141 7 7 7 7-3.141 7-7-3.141-7-7-7zM12.335 8c-2.468 0-3.712 1.77-3.712 3.244 0 .895.338 1.688 1.063 1.984.119.049.226.002.261-.129l.105-.418c.035-.129.021-.175-.074-.289-.209-.246-.344-.566-.344-1.019 0-1.312.982-2.487 2.558-2.487 1.396 0 2.161.853 2.161 1.99 0 1.498-.662 2.762-1.646 2.762-.543 0-.95-.449-.82-1.001.156-.658.459-1.368.459-1.843 0-.426-.229-.779-.7-.779-.556 0-1.002.574-1.002 1.344 0 .49.166.822.166.822l-.669 2.83c-.198.84-.029 1.87-.015 1.974.008.062.087.077.123.03.052-.067.713-.885.938-1.7.064-.23.366-1.427.366-1.427.18.344.707.646 1.268.646 1.67 0 2.803-1.521 2.803-3.56-.001-1.538-1.306-2.974-3.289-2.974z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
