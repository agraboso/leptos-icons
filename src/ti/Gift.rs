#[cfg(feature = "TiGift")]
use leptos::*;
#[cfg(feature = "TiGift")]
///This icon requires the feature `TiGift` to be enabled.
#[component]
pub fn Gift(
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
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 8h-2.352c.219-.457.352-.961.352-1.5 0-1.93-1.57-3.5-3.5-3.5-.979 0-1.864.407-2.5 1.058-.636-.651-1.521-1.058-2.5-1.058-1.93 0-3.5 1.57-3.5 3.5 0 .539.133 1.043.352 1.5h-2.352c-.553 0-1 .448-1 1v4c0 .552.447 1 1 1v5c0 1.654 1.346 3 3 3h10c1.654 0 3-1.346 3-3v-5c.553 0 1-.448 1-1v-4c0-.552-.447-1-1-1zm-1 4h-5v-2h5v2zm-8-5h2v1h-2v-1zm2 3v2h-2v-2h2zm1.5-5c.827 0 1.5.673 1.5 1.5s-.673 1.5-1.5 1.5c-.177 0-.344-.039-.5-.097v-.903c0-.521-.404-.937-.913-.982.202-.59.756-1.018 1.413-1.018zm-6.5 1.5c0-.827.673-1.5 1.5-1.5.657 0 1.211.428 1.413 1.018-.509.045-.913.461-.913.982v.903c-.156.058-.323.097-.5.097-.827 0-1.5-.673-1.5-1.5zm2 3.5v2h-5v-2h5zm-3 10c-.551 0-1-.449-1-1v-6h4v7h-3zm4 0v-7h2v7h-2zm6 0h-3v-7h4v6c0 .551-.449 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
