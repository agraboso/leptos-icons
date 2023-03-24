#[cfg(feature = "TiSupport")]
use leptos::*;
#[cfg(feature = "TiSupport")]
///This icon requires the feature `TiSupport` to be enabled.
#[component]
pub fn Support(
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
        "M12 3.5c-4.688 0-8.5 3.812-8.5 8.5s3.812 8.5 8.5 8.5 8.5-3.812 8.5-8.5-3.812-8.5-8.5-8.5zm6.5 8.5c0 1.064-.264 2.066-.718 2.956l-1.931-1.931c.088-.332.147-.674.147-1.025 0-.355-.062-.693-.147-1.021l1.932-1.932c.455.889.717 1.891.717 2.953zm-13 0c0-1.064.264-2.066.718-2.956l1.933 1.933c-.086.33-.147.668-.147 1.022 0 .353.062.69.147 1.021l-1.934 1.934c-.455-.89-.717-1.892-.717-2.954zm3.068-2.02l-1.775-1.775 1.414-1.414 1.775 1.775c-.584.345-1.068.83-1.414 1.414zm-1.777 5.813l1.773-1.773c.17.289.362.564.605.809s.52.438.807.607l-1.771 1.771-1.414-1.414zm3.795-2.379c-.377-.378-.585-.88-.584-1.414 0-1.104.896-2 1.998-2 1.104 0 2 .896 2 2.001.001.533-.207 1.035-.584 1.412-.755.757-2.073.757-2.83.001zm6.623-5.207l-1.775 1.775c-.345-.586-.828-1.069-1.412-1.416l1.773-1.773 1.414 1.414zm-2.378 6.619c.241-.242.435-.518.604-.803l1.771 1.771-1.414 1.414-1.772-1.772c.291-.17.567-.366.811-.61zm.125-8.608l-1.933 1.932c-.328-.088-.668-.15-1.023-.15s-.693.062-1.021.148l-1.932-1.932c.889-.455 1.891-.717 2.953-.717 1.064.001 2.066.265 2.956.719zm-5.912 11.564l1.933-1.933c.332.088.672.149 1.023.149s.691-.062 1.021-.147l1.932 1.932c-.889.455-1.891.717-2.953.717-1.064 0-2.066-.264-2.956-.718z"
        /> < title > { title } < / title > < / svg >
    }
}
