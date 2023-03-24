#[cfg(feature = "SiAmazoncloudwatch")]
use leptos::*;
#[cfg(feature = "SiAmazoncloudwatch")]
///This icon requires the feature `SiAmazoncloudwatch` to be enabled.
#[component]
pub fn Amazoncloudwatch(
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
        "M18.454 14.905c0-1.676-1.372-3.039-3.059-3.039-1.686 0-3.058 1.363-3.058 3.039 0 1.675 1.372 3.038 3.058 3.038 1.687 0 3.059-1.363 3.059-3.038Zm.862 0c0 2.147-1.759 3.894-3.92 3.894-2.162 0-3.92-1.747-3.92-3.894 0-2.148 1.758-3.895 3.92-3.895 2.161 0 3.92 1.747 3.92 3.895Zm3.617 5.87-3.004-2.688c-.242.34-.523.649-.834.926l2.999 2.687c.256.23.654.208.885-.046a.623.623 0 0 0-.046-.88Zm-7.538-1.206c2.59 0 4.696-2.092 4.696-4.664 0-2.573-2.106-4.665-4.696-4.665-2.589 0-4.696 2.092-4.696 4.665 0 2.572 2.107 4.664 4.696 4.664Zm8.224 2.658c-.293.323-.7.487-1.107.487a1.49 1.49 0 0 1-.995-.378L18.399 19.542a5.543 5.543 0 0 1-3.004.883c-3.064 0-5.557-2.476-5.557-5.52 0-3.044 2.493-5.521 5.557-5.521 3.065 0 5.558 2.477 5.558 5.52 0 .874-.21 1.697-.576 2.432l3.133 2.803c.608.546.657 1.482.11 2.088ZM3.977 7.454c0 .222.014.444.04.659a.426.426 0 0 1-.352.473C2.605 8.858.862 9.681.862 12.148c0 1.863 1.034 2.892 1.902 3.427.297.185.647.284 1.017.288l5.195.005v.856l-5.2-.005a2.815 2.815 0 0 1-1.469-.418C1.447 15.77 0 14.524 0 12.148c0-2.864 1.971-3.923 3.129-4.297a6.093 6.093 0 0 1-.013-.397c0-2.34 1.598-4.767 3.716-5.645 2.478-1.031 5.104-.52 7.022 1.367a7.048 7.048 0 0 1 1.459 2.116 2.79 2.79 0 0 1 1.78-.644c1.287 0 2.735.97 2.993 3.092 1.205.276 3.751 1.24 3.751 4.441 0 1.278-.403 2.333-1.199 3.137l-.614-.6c.632-.638.952-1.491.952-2.537 0-2.8-2.36-3.495-3.374-3.664a.43.43 0 0 1-.353-.496c-.141-1.738-1.18-2.517-2.156-2.517-.616 0-1.193.298-1.584.818a.431.431 0 0 1-.75-.111c-.353-.971-.861-1.788-1.511-2.426-1.663-1.636-3.936-2.079-6.084-1.186-1.787.74-3.187 2.873-3.187 4.855Z"
        /> < title > { title } < / title > < / svg >
    }
}
