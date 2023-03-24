#[cfg(feature = "SiNewyorktimes")]
use leptos::*;
#[cfg(feature = "SiNewyorktimes")]
///This icon requires the feature `SiNewyorktimes` to be enabled.
#[component]
pub fn Newyorktimes(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M21.272,14.815h-0.098c-0.747,2.049-2.335,3.681-4.363,4.483v-4.483l2.444-2.182l-2.444-2.182V7.397 c2.138,0.006,3.885-1.703,3.927-3.84c0-2.629-2.509-3.556-3.927-3.556c-0.367-0.007-0.734,0.033-1.091,0.12v0.131h0.556 c0.801-0.141,1.565,0.394,1.706,1.195C17.99,1.491,17.996,1.537,18,1.583c-0.033,0.789-0.7,1.401-1.488,1.367 c-0.02-0.001-0.041-0.002-0.061-0.004c-2.444,0-5.323-1.985-8.454-1.985C5.547,0.83,3.448,2.692,3.284,5.139 C3.208,6.671,4.258,8.031,5.76,8.346v-0.12C5.301,7.931,5.041,7.407,5.084,6.862c0.074-1.015,0.957-1.779,1.973-1.705 C7.068,5.159,7.08,5.16,7.091,5.161c2.629,0,6.872,2.182,9.501,2.182h0.098v3.142l-2.444,2.182l2.444,2.182v4.549 c-0.978,0.322-2.003,0.481-3.033,0.469c-1.673,0.084-3.318-0.456-4.614-1.516l4.429-1.985V7.451l-6.196,2.727 c0.592-1.75,1.895-3.168,3.589-3.905V6.175c-4.516,1.004-8.138,4.243-8.138,8.705c0,5.193,4.025,9.12,9.818,9.12 c6.011,0,8.727-4.363,8.727-8.814V14.815z M8.858,18.186c-1.363-1.362-2.091-3.235-2.007-5.16c-0.016-0.88,0.109-1.756,0.371-2.596 l2.051-0.938v8.476L8.858,18.186z"
        /> < title > { title } < / title > < / svg >
    }
}
