#[cfg(feature = "HiMdSolidPresentationChartLine")]
use leptos::*;
#[cfg(feature = "HiMdSolidPresentationChartLine")]
///This icon requires the feature `HiMdSolidPresentationChartLine` to be enabled.
#[component]
pub fn PresentationChartLine(
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
        "M1 2.75C1 2.33579 1.33579 2 1.75 2H18.25C18.6642 2 19 2.33579 19 2.75C19 3.16421 18.6642 3.5 18.25 3.5H18V12.25C18 13.7688 16.7688 15 15.25 15H14.1775L14.9759 18.0606C15.0805 18.4614 14.8403 18.8711 14.4395 18.9757C14.0387 19.0802 13.629 18.8401 13.5245 18.4393L13.4099 17.9999H6.5905L6.4759 18.4393C6.37134 18.8401 5.96167 19.0802 5.56087 18.9757C5.16007 18.8711 4.91991 18.4614 5.02447 18.0606L5.82289 15H4.75C3.23122 15 2 13.7688 2 12.25V3.5H1.75C1.33579 3.5 1 3.16421 1 2.75ZM7.37309 15L6.9818 16.4999H13.0186L12.6273 15H7.37309ZM14.8625 6.0688C15.1031 6.40599 15.0248 6.87436 14.6876 7.11493C13.4276 8.0139 12.2896 9.05711 11.2905 10.2134C11.1542 10.3711 10.9585 10.465 10.7503 10.4726C10.542 10.4801 10.34 10.4007 10.1927 10.2534L8.5 8.56073L6.28033 10.7804C5.98744 11.0733 5.51256 11.0733 5.21967 10.7804C4.92678 10.4875 4.92678 10.0126 5.21967 9.71973L7.96967 6.96973C8.26256 6.67684 8.73744 6.67684 9.03033 6.96973L10.6938 8.63325C11.6347 7.62514 12.679 6.70534 13.8164 5.89386C14.1536 5.65328 14.6219 5.73161 14.8625 6.0688Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
