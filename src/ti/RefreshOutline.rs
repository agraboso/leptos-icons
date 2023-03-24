#[cfg(feature = "TiRefreshOutline")]
use leptos::*;
#[cfg(feature = "TiRefreshOutline")]
///This icon requires the feature `TiRefreshOutline` to be enabled.
#[component]
pub fn RefreshOutline(
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
        "M17.368 4.998c-.488 0-1.2.145-1.956.773-1.036-.489-2.189-.771-3.412-.771-4.418 0-8 3.582-8 8s3.582 8 8 8c4.312 0 8-3.316 8-8v-4.936c-.016-2.111-1.375-3.066-2.632-3.066zm.632 8.002h-5.128c-1.134 0-1.407-.561-.604-1.363l1.448-1.402c-.562-.371-1.22-.549-1.909-.549-.93 0-1.805.375-2.464 1.033-.657.656-1.02 1.537-1.02 2.467 0 .933.362 1.811 1.02 2.469.659.658 1.534 1.021 2.464 1.021s1.805-.36 2.465-1.019c.177-.18.334-.372.468-.579.222-.345.596-.533.979-.533.216 0 .433.06.625.185.54.346.697 1.063.351 1.604-.223.344-.484.668-.78.965-1.097 1.099-2.556 1.703-4.106 1.703-1.55 0-3.009-.604-4.104-1.701-1.097-1.096-1.701-2.555-1.701-4.106 0-1.551.604-3.012 1.702-4.104 1.096-1.098 2.554-1.7 4.104-1.7 1.311 0 2.551.436 3.566 1.229l1.154-1.158c.311-.312.602-.461.841-.461.377 0 .627.372.632 1.065v4.934zm-7.08.05c.162.392.63.95 1.952.95h1.299s-.21.504-.614.907-1.086.745-1.75.745-1.289-.246-1.758-.715c-.468-.47-.727-1.088-.727-1.752s.258-1.139.726-1.604c.472-.472 1.097-.581 1.759-.581l-.246.123c-.935.934-.803 1.536-.641 1.927z"
        /> < title > { title } < / title > < / svg >
    }
}
