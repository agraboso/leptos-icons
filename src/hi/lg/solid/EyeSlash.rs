#[cfg(feature = "HiLgSolidEyeSlash")]
use leptos::*;
#[cfg(feature = "HiLgSolidEyeSlash")]
///This icon requires the feature `HiLgSolidEyeSlash` to be enabled.
#[component]
pub fn EyeSlash(
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
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3.53033 2.46967C3.23744 2.17678 2.76256 2.17678 2.46967 2.46967C2.17678 2.76256 2.17678 3.23744 2.46967 3.53033L20.4697 21.5303C20.7626 21.8232 21.2374 21.8232 21.5303 21.5303C21.8232 21.2374 21.8232 20.7626 21.5303 20.4697L3.53033 2.46967Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M22.6759 12.5533C22.1319 14.1887 21.2226 15.6575 20.0447 16.8627L16.9462 13.7642C17.1429 13.2129 17.25 12.6189 17.25 12C17.25 9.1005 14.8995 6.75 12 6.75C11.3811 6.75 10.7871 6.8571 10.2358 7.05379L7.75898 4.577C9.06783 4.04381 10.4998 3.75 12.0005 3.75C16.9708 3.75 21.1864 6.97271 22.6755 11.4405C22.7959 11.8015 22.796 12.1922 22.6759 12.5533Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.75 12C15.75 12.1802 15.7373 12.3574 15.7127 12.5307L11.4693 8.28727C11.6426 8.26271 11.8198 8.25 12 8.25C14.0711 8.25 15.75 9.92893 15.75 12Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12.5307 15.7127L8.28727 11.4693C8.26271 11.6426 8.25 11.8198 8.25 12C8.25 14.0711 9.92893 15.75 12 15.75C12.1802 15.75 12.3574 15.7373 12.5307 15.7127Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.75 12C6.75 11.3811 6.8571 10.7871 7.05379 10.2358L3.95492 7.1369C2.77687 8.34222 1.86747 9.81114 1.32341 11.4467C1.20328 11.8078 1.2034 12.1985 1.32374 12.5595C2.81284 17.0273 7.02847 20.25 11.9988 20.25C13.4997 20.25 14.9318 19.9561 16.2408 19.4228L13.7642 16.9462C13.2129 17.1429 12.6189 17.25 12 17.25C9.1005 17.25 6.75 14.8995 6.75 12Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
