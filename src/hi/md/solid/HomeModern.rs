#[cfg(feature = "HiMdSolidHomeModern")]
use leptos::*;
#[cfg(feature = "HiMdSolidHomeModern")]
///This icon requires the feature `HiMdSolidHomeModern` to be enabled.
#[component]
pub fn HomeModern(
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
        "M14.9156 2.40414C15.1067 2.77164 14.9637 3.22447 14.5962 3.41557L14 3.72558V17.0002C14 17.5524 13.5523 18.0002 13 18.0002H10.74C10.3258 18.0002 9.99 17.6644 9.99 17.2502V13.7502C9.99 13.3359 9.65421 13.0002 9.24 13.0002H6.75C6.33579 13.0002 6 13.3359 6 13.7502V17.2502C6 17.6644 5.66421 18.0002 5.25 18.0002H1.75C1.33579 18.0002 1 17.6644 1 17.2502C1 16.8359 1.33579 16.5002 1.75 16.5002H2V9.95741C1.65296 10.0805 1.25946 9.93216 1.08474 9.59617C0.893644 9.22867 1.03664 8.77584 1.40414 8.58474L2 8.27489V5.75015C2 5.33594 2.33579 5.00015 2.75 5.00015C3.16421 5.00015 3.5 5.33594 3.5 5.75015V7.49489L13.9041 2.08474C14.2716 1.89364 14.7245 2.03664 14.9156 2.40414Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.8615 8.56938C16.0854 8.4334 16.3642 8.42419 16.5967 8.54508L18.596 9.58475C18.9635 9.77584 19.1065 10.2287 18.9154 10.5962C18.7407 10.9321 18.3473 11.0805 18.0003 10.9575V16.5002H18.25C18.6642 16.5002 19 16.8359 19 17.2502C19 17.6644 18.6642 18.0002 18.25 18.0002H16.25C16.0511 18.0002 15.8603 17.9211 15.7196 17.7805C15.579 17.6398 15.5 17.449 15.5 17.2501L15.5006 9.21043C15.5007 8.94839 15.6375 8.70536 15.8615 8.56938Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
