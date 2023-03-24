#[cfg(feature = "ImIe")]
use leptos::*;
#[cfg(feature = "ImIe")]
///This icon requires the feature `ImIe` to be enabled.
#[component]
pub fn Ie(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M11.472 9.825h3.688c0.028-0.256 0.040-0.517 0.040-0.784 0-1.253-0.336-2.429-0.924-3.442 0.607-1.614 0.586-2.984-0.227-3.803-0.773-0.77-2.848-0.645-5.194 0.394-0.174-0.013-0.349-0.020-0.526-0.020-3.22 0-5.921 2.216-6.667 5.201 1.010-1.293 2.072-2.231 3.492-2.913-0.129 0.121-0.882 0.87-1.009 0.996-3.743 3.742-4.923 8.63-3.653 9.9 0.965 0.965 2.715 0.802 4.725-0.182 0.934 0.476 1.992 0.744 3.113 0.744 3.018 0 5.575-1.942 6.501-4.648h-3.717c-0.511 0.943-1.512 1.586-2.66 1.586s-2.148-0.642-2.66-1.586c-0.227-0.426-0.358-0.915-0.358-1.432v-0.011h6.035zM5.442 8.013c0.085-1.517 1.347-2.728 2.887-2.728s2.802 1.21 2.887 2.728h-5.774zM14.015 2.559c0.524 0.529 0.511 1.503 0.063 2.719-0.768-1.17-1.883-2.093-3.2-2.619 1.408-0.604 2.553-0.684 3.137-0.1zM1.461 15.113c-0.668-0.669-0.467-2.072 0.394-3.763 0.536 1.504 1.581 2.767 2.927 3.581-1.491 0.677-2.712 0.792-3.321 0.182z"
        /> < title > { title } < / title > < / svg >
    }
}
