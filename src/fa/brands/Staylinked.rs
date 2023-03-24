#[cfg(feature = "FaBrandsStaylinked")]
use leptos::*;
#[cfg(feature = "FaBrandsStaylinked")]
///This icon requires the feature `FaBrandsStaylinked` to be enabled.
#[component]
pub fn Staylinked(
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
        stroke_witdh = "0" style = style viewBox = "0 0 440 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M382.7 292.5l2.7 2.7-170-167.3c-3.5-3.5-9.7-3.7-13.8-.5L144.3 171c-4.2 3.2-4.6 8.7-1.1 12.2l68.1 64.3c3.6 3.5 9.9 3.7 14 .5l.1-.1c4.1-3.2 10.4-3 14 .5l84 81.3c3.6 3.5 3.2 9-.9 12.2l-93.2 74c-4.2 3.3-10.5 3.1-14.2-.4L63.2 268c-3.5-3.5-9.7-3.7-13.9-.5L3.5 302.4c-4.2 3.2-4.7 8.7-1.2 12.2L211 510.7s7.4 6.8 17.3-.8l198-163.9c4-3.2 4.4-8.7.7-12.2zm54.5-83.4L226.7 2.5c-1.5-1.2-8-5.5-16.3 1.1L3.6 165.7c-4.2 3.2-4.8 8.7-1.2 12.2l42.3 41.7 171.7 165.1c3.7 3.5 10.1 3.7 14.3.4l50.2-38.8-.3-.3 7.7-6c4.2-3.2 4.6-8.7.9-12.2l-57.1-54.4c-3.6-3.5-10-3.7-14.2-.5l-.1.1c-4.2 3.2-10.5 3.1-14.2-.4L109 180.8c-3.6-3.5-3.1-8.9 1.1-12.2l92.2-71.5c4.1-3.2 10.3-3 13.9.5l160.4 159c3.7 3.5 10 3.7 14.1.5l45.8-35.8c4.1-3.2 4.4-8.7.7-12.2z"
        /> < title > { title } < / title > < / svg >
    }
}
