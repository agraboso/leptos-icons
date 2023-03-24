#[cfg(feature = "SiPolkadot")]
use leptos::*;
#[cfg(feature = "SiPolkadot")]
///This icon requires the feature `SiPolkadot` to be enabled.
#[component]
pub fn Polkadot(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M12 0C6.985 0 2.886 4.075 2.886 9.114a9.55 9.55 0 0 0 .482 2.966c.217.651.94 1.013 1.615.796.651-.217 1.013-.94.796-1.616a6.42 6.42 0 0 1-.362-2.338c.097-3.4 2.846-6.197 6.245-6.366 3.786-.193 6.92 2.821 6.92 6.558a6.579 6.579 0 0 1-6.196 6.559s-1.278.072-1.905.169c-.314.048-.555.096-.723.12-.073.024-.145-.048-.121-.12l.217-1.061 1.182-5.45a1.264 1.264 0 0 0-.965-1.494 1.264 1.264 0 0 0-1.495.964s-2.845 13.262-2.87 13.406c-.144.676.29 1.35.965 1.495.675.145 1.35-.289 1.495-.964.024-.145.41-1.905.41-1.905a3.203 3.203 0 0 1 2.7-2.508c.29-.048 1.423-.12 1.423-.12 4.702-.362 8.415-4.292 8.415-9.09C21.114 4.074 17.015 0 12 0Zm.651 20.978a1.518 1.518 0 0 0-1.808 1.181c-.17.82.337 1.64 1.181 1.809.82.168 1.64-.338 1.809-1.182.168-.844-.338-1.64-1.182-1.808z"
        /> < title > { title } < / title > < / svg >
    }
}
