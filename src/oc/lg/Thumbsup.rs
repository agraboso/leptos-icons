#[cfg(feature = "OcLgThumbsup")]
use leptos::*;
#[cfg(feature = "OcLgThumbsup")]
///This icon requires the feature `OcLgThumbsup` to be enabled.
#[component]
pub fn Thumbsup(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12.596 2.043c1.075.076 2.059.281 2.743.956.698.688.92 1.696.92 2.941 0 .432-.057.955-.117 1.438-.026.2-.051.392-.076.572l-.056.429h2.05c.752 0 1.446.108 2.036.404.612.306 1.062.787 1.355 1.431.551 1.214.542 3.008.223 5.394l-.051.39c-.134 1.01-.248 1.872-.396 2.58-.166.795-.394 1.496-.816 2.05-.89 1.168-2.395 1.372-4.583 1.372-2.331 0-4.08-.418-5.544-.824l-.602-.17c-1.023-.29-1.852-.526-2.69-.586A1.75 1.75 0 0 1 5.25 22h-1.5A1.75 1.75 0 0 1 2 20.25V9.75C2 8.784 2.784 8 3.75 8h1.5a1.75 1.75 0 0 1 1.746 1.633 1.85 1.85 0 0 0 .523-.131c.961-.415 2.774-1.534 2.774-4.2V4.249c0-1.22 1.002-2.298 2.303-2.206ZM7 18.918c1.059.064 2.079.355 3.118.652l.568.16c1.406.39 3.006.77 5.142.77 2.277 0 3.004-.274 3.39-.781.216-.283.388-.718.54-1.448.136-.65.242-1.45.379-2.477l.05-.384c.32-2.4.253-3.795-.102-4.575-.16-.352-.375-.568-.66-.711-.305-.153-.74-.245-1.365-.245h-2.37c-.681 0-1.293-.57-1.211-1.328.026-.243.065-.537.105-.834l.07-.527c.06-.482.105-.921.105-1.25 0-1.125-.213-1.617-.473-1.873-.275-.27-.774-.455-1.795-.528-.351-.024-.698.274-.698.71v1.053c0 3.55-2.488 5.063-3.68 5.577-.372.16-.754.232-1.113.26ZM3.75 20.5h1.5a.25.25 0 0 0 .25-.25V9.75a.25.25 0 0 0-.25-.25h-1.5a.25.25 0 0 0-.25.25v10.5c0 .138.112.25.25.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
