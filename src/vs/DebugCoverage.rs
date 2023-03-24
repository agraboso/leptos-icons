#[cfg(feature = "VsDebugCoverage")]
use leptos::*;
#[cfg(feature = "VsDebugCoverage")]
///This icon requires the feature `VsDebugCoverage` to be enabled.
#[component]
pub fn DebugCoverage(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M5 2.41L5.78 2L14.78 8V8.83L9 12.6833V11.4826L13.6 8.42L6 3.35V7H5V2.41Z" /><
        path xmlns = "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule =
        "evenodd" d =
        "M6.13041 12.1236C5.94796 12.3032 5.80777 12.503 5.70927 12.7232C5.61337 12.9427 5.56516 13.181 5.56516 13.4389V14.3007C5.56516 14.3973 5.54694 14.4886 5.51016 14.5741C5.4738 14.6587 5.42387 14.7328 5.36036 14.7961C5.29687 14.8594 5.2225 14.9091 5.13774 14.9453C5.05203 14.9819 4.96049 15 4.86366 15H3.9988C3.90197 15 3.81043 14.9819 3.72472 14.9453C3.63996 14.9091 3.5656 14.8594 3.5021 14.7961C3.4386 14.7328 3.38866 14.6587 3.3523 14.5741C3.31552 14.4886 3.2973 14.3973 3.2973 14.3007V13.4389C3.2973 13.1811 3.248 12.9428 3.1499 12.7234C3.05368 12.5033 2.91448 12.3031 2.73205 12.1236C2.49791 11.8926 2.31713 11.6346 2.19041 11.35C2.0633 11.0644 2 10.7552 2 10.4228C2 10.2005 2.02876 9.98586 2.08641 9.77906C2.14392 9.57279 2.22565 9.38 2.33166 9.20087C2.43754 9.01972 2.56419 8.85567 2.71156 8.70884C2.85886 8.56206 3.02231 8.4359 3.20182 8.33042C3.38373 8.22488 3.57848 8.14334 3.78542 8.08606C3.99288 8.02865 4.2082 8 4.43123 8C4.65426 8 4.86959 8.02865 5.07704 8.08606C5.28398 8.14334 5.4774 8.22475 5.65714 8.33035C5.8389 8.43582 6.00353 8.56199 6.15091 8.70884C6.29827 8.85567 6.42492 9.01972 6.53079 9.20086C6.63681 9.37999 6.71854 9.57278 6.77605 9.77906C6.8337 9.98586 6.86246 10.2005 6.86246 10.4228C6.86246 10.7552 6.79916 11.0644 6.67206 11.35C6.54533 11.6346 6.36456 11.8926 6.13041 12.1236ZM5.02703 13.1154H3.83544V14.3007C3.83544 14.3443 3.8508 14.3814 3.88401 14.4145C3.91724 14.4476 3.95465 14.4631 3.9988 14.4631H4.86366C4.90781 14.4631 4.94523 14.4476 4.97845 14.4145C5.01166 14.3814 5.02703 14.3443 5.02703 14.3007V13.1154Z"
        /> < title > { title } < / title > < / svg >
    }
}
