#[cfg(feature = "FaSolidPlateWheat")]
use leptos::*;
#[cfg(feature = "FaSolidPlateWheat")]
///This icon requires the feature `FaSolidPlateWheat` to be enabled.
#[component]
pub fn PlateWheat(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M176 32c44.2 0 80 35.8 80 80v16c0 8.8-7.2 16-16 16c-44.2 0-80-35.8-80-80V48c0-8.8 7.2-16 16-16zM56 64h48c13.3 0 24 10.7 24 24s-10.7 24-24 24H56c-13.3 0-24-10.7-24-24s10.7-24 24-24zM24 136H136c13.3 0 24 10.7 24 24s-10.7 24-24 24H24c-13.3 0-24-10.7-24-24s10.7-24 24-24zm8 96c0-13.3 10.7-24 24-24h48c13.3 0 24 10.7 24 24s-10.7 24-24 24H56c-13.3 0-24-10.7-24-24zM272 48c0-8.8 7.2-16 16-16c44.2 0 80 35.8 80 80v16c0 8.8-7.2 16-16 16c-44.2 0-80-35.8-80-80V48zM400 32c44.2 0 80 35.8 80 80v16c0 8.8-7.2 16-16 16c-44.2 0-80-35.8-80-80V48c0-8.8 7.2-16 16-16zm80 160v16c0 44.2-35.8 80-80 80c-8.8 0-16-7.2-16-16V256c0-44.2 35.8-80 80-80c8.8 0 16 7.2 16 16zM352 176c8.8 0 16 7.2 16 16v16c0 44.2-35.8 80-80 80c-8.8 0-16-7.2-16-16V256c0-44.2 35.8-80 80-80zm-96 16v16c0 44.2-35.8 80-80 80c-8.8 0-16-7.2-16-16V256c0-44.2 35.8-80 80-80c8.8 0 16 7.2 16 16zM3.5 347.6C1.6 332.9 13 320 27.8 320H484.2c14.8 0 26.2 12.9 24.4 27.6C502.3 397.8 464.2 437 416 446v2c0 17.7-14.3 32-32 32H128c-17.7 0-32-14.3-32-32v-2c-48.2-9-86.3-48.2-92.5-98.4z"
        /> < title > { title } < / title > < / svg >
    }
}
