#[cfg(feature = "RiLogosLineWechat2")]
use leptos::*;
#[cfg(feature = "RiLogosLineWechat2")]
///This icon requires the feature `RiLogosLineWechat2` to be enabled.
#[component]
pub fn Wechat2(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M8.667 11.511a1.276 1.276 0 0 1-1.285-1.285c0-.718.567-1.286 1.285-1.286.717 0 1.285.568 1.285 1.286 0 .717-.568 1.285-1.285 1.285zm6.666 0a1.276 1.276 0 0 1-1.285-1.285c0-.718.568-1.286 1.285-1.286.718 0 1.285.568 1.285 1.286 0 .717-.567 1.285-1.285 1.285zm-8.51 7.704l.715-.436a4 4 0 0 1 2.705-.536c.212.033.386.059.52.076.406.054.82.081 1.237.081 4.42 0 7.9-3.022 7.9-6.6S16.42 5.2 12 5.2s-7.9 3.022-7.9 6.6c0 1.366.5 2.673 1.432 3.781.048.057.12.137.214.235a4 4 0 0 1 1.101 3.102l-.025.297zm-.63 2.727a1 1 0 0 1-1.527-.93l.188-2.26a2 2 0 0 0-.55-1.551A6.993 6.993 0 0 1 4 16.868C2.806 15.447 2.1 13.695 2.1 11.8c0-4.75 4.432-8.6 9.9-8.6s9.9 3.85 9.9 8.6-4.432 8.6-9.9 8.6c-.51 0-1.01-.033-1.499-.098a23.61 23.61 0 0 1-.569-.084 2 2 0 0 0-1.353.268l-2.387 1.456z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
