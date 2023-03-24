#[cfg(feature = "HiMdSolidLightBulb")]
use leptos::*;
#[cfg(feature = "HiMdSolidLightBulb")]
///This icon requires the feature `HiMdSolidLightBulb` to be enabled.
#[component]
pub fn LightBulb(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10 1C6.68629 1 4 3.68629 4 7C4 8.86519 4.85197 10.532 6.18519 11.6313C7.23747 12.4989 8 13.4427 8 14.4557V15.1003C8 15.4459 8.23625 15.7468 8.57205 15.8289C9.03046 15.9408 9.50883 16 10 16C10.4912 16 10.9695 15.9408 11.428 15.8289C11.7637 15.7468 12 15.4459 12 15.1003V14.4557C12 13.4427 12.7625 12.4989 13.8148 11.6313C15.148 10.532 16 8.86519 16 7C16 3.68629 13.3137 1 10 1Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.8628 17.4144C8.45329 17.3522 8.07086 17.6337 8.00862 18.0432C7.94638 18.4527 8.22789 18.8352 8.6374 18.8974C9.08221 18.965 9.53731 19 10.0001 19C10.4629 19 10.918 18.965 11.3628 18.8974C11.7723 18.8352 12.0538 18.4527 11.9916 18.0432C11.9293 17.6337 11.5469 17.3522 11.1374 17.4144C10.7669 17.4708 10.3872 17.5 10.0001 17.5C9.61302 17.5 9.23326 17.4708 8.8628 17.4144Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
