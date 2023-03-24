#[cfg(feature = "BiLogosInternetExplorer")]
use leptos::*;
#[cfg(feature = "BiLogosInternetExplorer")]
///This icon requires the feature `BiLogosInternetExplorer` to be enabled.
#[component]
pub fn InternetExplorer(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20.154 8.514c.85-2.039.906-3.738-.113-4.757-1.133-1.132-3.852-.792-6.737.565h-.341c-2.037 0-4.02.738-5.492 2.039-1.244 1.133-2.15 2.605-2.547 4.246.283-.337 1.869-2.207 3.68-3.226.058 0 .51-.283.51-.283-.057 0-.905.85-1.074 1.02-3.965 4.076-6.285 10.248-4.473 12.059 1.188 1.189 3.34.906 5.831-.452 1.075.51 2.265.735 3.565.735 1.699 0 3.285-.453 4.643-1.359 1.416-.906 2.436-2.319 3.059-3.962h-4.416c-.566 1.075-1.869 1.812-3.227 1.812-1.926 0-3.512-1.586-3.568-3.397v-.169h11.551v-.17c0-.283.057-.623.057-.848-.001-1.362-.341-2.664-.908-3.853zM4.811 19.838c-.906-.906-.621-2.662.453-4.756.51 1.416 1.359 2.662 2.435 3.567.341.282.735.565 1.133.792-1.814.963-3.285 1.133-4.021.397zm11.606-8.606H9.51v-.057c.113-1.756 1.754-3.285 3.681-3.285 1.812 0 3.284 1.416 3.397 3.285v.057h-.171zm3.454-3.171a6.948 6.948 0 0 0-1.303-1.585 8.445 8.445 0 0 0-2.718-1.699c1.813-.849 3.341-.961 4.134-.169.623.736.567 1.981-.113 3.453 0 .057 0 .057 0 0 0 .057 0 .057 0 0z"
        /> < title > { title } < / title > < / svg >
    }
}
