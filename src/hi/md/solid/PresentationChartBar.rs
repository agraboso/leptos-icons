#[cfg(feature = "HiMdSolidPresentationChartBar")]
use leptos::*;
#[cfg(feature = "HiMdSolidPresentationChartBar")]
///This icon requires the feature `HiMdSolidPresentationChartBar` to be enabled.
#[component]
pub fn PresentationChartBar(
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
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M1 2.75C1 2.33579 1.33579 2 1.75 2H18.25C18.6642 2 19 2.33579 19 2.75C19 3.16421 18.6642 3.5 18.25 3.5H18V12.25C18 13.7688 16.7688 15 15.25 15H14.1775L14.9759 18.0606C15.0805 18.4614 14.8403 18.8711 14.4395 18.9757C14.0387 19.0802 13.629 18.8401 13.5245 18.4393L13.4099 17.9999H6.5905L6.4759 18.4393C6.37134 18.8401 5.96167 19.0802 5.56087 18.9757C5.16007 18.8711 4.91991 18.4614 5.02447 18.0606L5.82289 15H4.75C3.23122 15 2 13.7688 2 12.25V3.5H1.75C1.33579 3.5 1 3.16421 1 2.75ZM7.37309 15L6.9818 16.4999H13.0186L12.6273 15H7.37309ZM13.25 5C13.6642 5 14 5.33579 14 5.75V11.25C14 11.6642 13.6642 12 13.25 12C12.8358 12 12.5 11.6642 12.5 11.25V5.75C12.5 5.33579 12.8358 5 13.25 5ZM6.75 9C7.16421 9 7.5 9.33579 7.5 9.75V11.25C7.5 11.6642 7.16421 12 6.75 12C6.33579 12 6 11.6642 6 11.25V9.75C6 9.33579 6.33579 9 6.75 9ZM10.75 7.75C10.75 7.33579 10.4142 7 10 7C9.58579 7 9.25 7.33579 9.25 7.75V11.25C9.25 11.6642 9.58579 12 10 12C10.4142 12 10.75 11.6642 10.75 11.25V7.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
