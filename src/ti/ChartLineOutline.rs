#[cfg(feature = "TiChartLineOutline")]
use leptos::*;
#[cfg(feature = "TiChartLineOutline")]
///This icon requires the feature `TiChartLineOutline` to be enabled.
#[component]
pub fn ChartLineOutline(
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
        stroke_witdh = "0" style = style version = "1.2" baseProfile = "tiny" width =
        "24" height = "24" viewBox = "0 0 24 24" width = { size.clone() } height = { size
        } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.999 17c-.677 0-1.342-.234-1.873-.658-.626-.501-1.019-1.215-1.107-2.011-.089-.796.138-1.58.639-2.206l4-5c.978-1.225 2.883-1.471 4.143-.523l1.674 1.254 2.184-2.729c.571-.716 1.425-1.127 2.342-1.127.679 0 1.343.232 1.873.657.626.501 1.021 1.216 1.108 2.013s-.14 1.58-.641 2.204l-4 5c-.977 1.226-2.882 1.471-4.143.526l-1.674-1.256-2.184 2.729c-.57.716-1.424 1.127-2.341 1.127zm4.001-9c-.306 0-.591.137-.781.374l-4 5.001c-.167.208-.243.471-.213.734.03.266.161.504.369.67.228.183.465.221.624.221.306 0 .591-.137.782-.376l3.395-4.244 3.224 2.42c.42.316 1.056.231 1.381-.176l4-5.001c.167-.208.242-.469.213-.734-.03-.266-.161-.504-.369-.67-.227-.182-.464-.22-.624-.22-.306 0-.591.137-.782.376l-3.395 4.242-3.224-2.417c-.175-.132-.382-.2-.6-.2zM19 21h-14c-.552 0-1-.447-1-1s.448-1 1-1h14c.552 0 1 .447 1 1s-.448 1-1 1z"
        /> < title > { title } < / title > < / svg >
    }
}
