#[cfg(feature = "HiLgOutlineHandThumbDown")]
use leptos::*;
#[cfg(feature = "HiLgOutlineHandThumbDown")]
///This icon requires the feature `HiLgOutlineHandThumbDown` to be enabled.
#[component]
pub fn HandThumbDown(
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
        "M7.5 15L9.75 15M17.7745 5.25C17.7851 5.30001 17.802 5.34962 17.8258 5.3978C18.4175 6.59708 18.75 7.94722 18.75 9.375C18.75 10.8618 18.3895 12.2643 17.7511 13.5M17.7745 5.25C17.6975 4.88534 17.9575 4.5 18.3493 4.5L19.2571 4.5C20.1458 4.5 20.9701 5.01802 21.2294 5.86805C21.5679 6.97738 21.75 8.15493 21.75 9.375C21.75 10.9275 21.4552 12.4111 20.9185 13.7729C20.6135 14.547 19.8327 15 19.0006 15H17.9479C17.476 15 17.2027 14.4441 17.4477 14.0407C17.5548 13.8644 17.6561 13.684 17.7511 13.5M17.7745 5.25L16.4803 5.25C15.9966 5.25 15.5161 5.17203 15.0572 5.01908L11.9428 3.98093C11.4839 3.82798 11.0034 3.75 10.5198 3.75L6.50377 3.75C5.88581 3.75 5.2866 3.99749 4.899 4.47878C3.24188 6.53642 2.25 9.15238 2.25 12C2.25 12.4341 2.27306 12.8629 2.31801 13.2851C2.4267 14.306 3.34564 15 4.37227 15L7.49809 15C8.11638 15 8.48896 15.724 8.22337 16.2823C7.75956 17.2574 7.5 18.3484 7.5 19.5C7.5 20.7426 8.50736 21.75 9.75 21.75C10.1642 21.75 10.5 21.4142 10.5 21V20.3666C10.5 19.7941 10.6092 19.2269 10.8219 18.6954C11.1257 17.9357 11.7523 17.3644 12.4745 16.9798C13.5883 16.3866 14.5627 15.5662 15.3359 14.5803C15.8335 13.9458 16.5611 13.5 17.3674 13.5H17.7511"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
