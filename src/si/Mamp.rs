#[cfg(feature = "SiMamp")]
use leptos::*;
#[cfg(feature = "SiMamp")]
///This icon requires the feature `SiMamp` to be enabled.
#[component]
pub fn Mamp(
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
        "m.102 9.629c.245-1.695 2.139-6.979 7.454-7.98 3.137-.592 7.005 1.034 9.72 3.788-1.354.929-2.259 1.732-2.944 2.708.511 1.892 2.018 2.663 3.078 2.984-1.175-.711-1.917-1.381-2.376-2.812 2.513-2.988 5.205-3.954 5.432-3.978 2.102 1.677 3.586 4.735 3.532 7.955-.062 3.728-2.09 7.569-4.415 9.121-.318.151-1.053-.25-1.153-.687.476-1.105 1.601-3.795 1.74-5.806.131-1.928-1.146-3.061-2.219-1.667-1.489 2.494-2.139 5.592-2.789 8.934-1.095.313-2.952.392-3.756-.036-.101-2.068.381-5.601-.991-6.84-.588-.533-1.169-.285-1.562.205-.992 1.227-.972 4.414-.78 6.479-1.109.686-2.99.543-4.179.117-.477-3.245-1.597-7.202-2.512-10.23-.255 1.216-.379 2.664-.34 3.908-.611-.498-1.308-3.116-.94-6.163z"
        /> < title > { title } < / title > < / svg >
    }
}
