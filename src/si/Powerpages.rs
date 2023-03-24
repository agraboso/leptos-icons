#[cfg(feature = "SiPowerpages")]
use leptos::*;
#[cfg(feature = "SiPowerpages")]
///This icon requires the feature `SiPowerpages` to be enabled.
#[component]
pub fn Powerpages(
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
        "M5.42 14.624 3.585 16a1.258 1.258 0 0 0 0 2.014l7.66 5.745a1.257 1.257 0 0 0 1.51 0l2.612-1.959a1.841 1.841 0 0 1-.828-.337c-3.081-2.223-6.1-4.531-9.119-6.839Zm13.16-4.622 4.925 3.694c.66.503.66 1.497 0 2.001l-7.155 5.366a1.259 1.259 0 0 1-1.511 0l-5.693-4.27c.294-.038.58-.15.828-.337l8.606-6.454Zm-18.077.309a1.259 1.259 0 0 1 .001-2.014L11.245.241a1.257 1.257 0 0 1 1.51 0l7.661 5.745c.671.503.671 1.51 0 2.013L9.674 16.056a1.262 1.262 0 0 1-1.511 0l-7.66-5.745Z"
        /> < title > { title } < / title > < / svg >
    }
}
