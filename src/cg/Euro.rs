#[cfg(feature = "CgEuro")]
use leptos::*;
#[cfg(feature = "CgEuro")]
///This icon requires the feature `CgEuro` to be enabled.
#[component]
pub fn Euro(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18.5535 15.5355C17.6487 16.4404 16.3987 17 15.018 17C13.1416 17 11.5066 15.9664 10.6513 14.4374H13.0323L13.7284 12.5625H10.0493C10.0286 12.3779 10.018 12.1902 10.018 12C10.018 11.8098 10.0286 11.6221 10.0493 11.4374H14.3362L15.0324 9.5625H10.6514C11.5066 8.0336 13.1416 7 15.018 7C16.3987 7 17.6487 7.55964 18.5535 8.46447L19.9677 7.05025C18.701 5.7835 16.951 5 15.018 5C12.0092 5 9.44381 6.89827 8.45407 9.5625H6.03241L5.33624 11.4374H8.04028C8.02552 11.623 8.018 11.8106 8.018 12C8.018 12.1894 8.02552 12.3769 8.04027 12.5625H4.72845L4.03229 14.4374H8.45404C9.44377 17.1017 12.0092 19 15.018 19C16.951 19 18.701 18.2165 19.9677 16.9497L18.5535 15.5355Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
