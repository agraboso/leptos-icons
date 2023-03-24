#[cfg(feature = "SiRocksdb")]
use leptos::*;
#[cfg(feature = "SiRocksdb")]
///This icon requires the feature `SiRocksdb` to be enabled.
#[component]
pub fn Rocksdb(
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
        "m19.299 2.519-2.102.985c.006.005.006.014.006.02a.81.81 0 0 0 .303.337c-2.557 1.32-3.643 3.104-3.643 3.104L4.09 12.228a1.183 1.183 0 0 1-1.598-.482 1.183 1.183 0 0 1 .483-1.596l2.771-1.492A2.226 2.226 0 0 0 6.912 6.84l-1.664.894-2.771 1.494a2.23 2.23 0 0 0 2.109 3.927l1.25-.674c-.09.493-.05 1.019.146 1.516.066.17.11.277.11.277.443.96-.14 1.713-.903 1.713h-.242l-.879 3.828a.93.93 0 0 0-.441-.255l-.438 1.802a.923.923 0 0 0 1.061-.521l1.992-3.578c2.522-.246 3.85-1.954 3.79-4.36 2.611.16 6.093-1.48 7.64-3.726 1.175 1.492 3.395 1.28 4.385.902l.19-.064a.93.93 0 0 0-.11.44l1.857-.02a.933.933 0 0 0-.94-.913h-.01l-3.192-.275-.73-1.914v-2.03a.982.982 0 0 0 .483.126c.527 0 .96-.406 1.01-.918l.764-.508-.035-1.03a.19.19 0 0 0-.192-.187l-.955-.014a.73.73 0 0 0-.898-.252zm.25.508c.166 0 .303.135.303.3a.305.305 0 0 1-.303.304.301.301 0 0 1-.295-.303c0-.166.13-.3.295-.3zM5.48 4.567a.528.528 0 0 0-.232.686l.127.22.924-.495-.12-.221a.528.528 0 0 0-.699-.19zm12.487.235c.156 0 .281.144.281.144s-.045.056-.115.096c.11.045.185.115.185.115s-.075.072-.185.117a.47.47 0 0 1 .115.094s-.125.147-.281.147c-.156 0-.281-.147-.281-.147s.044-.053.115-.094a.558.558 0 0 1-.186-.117s.075-.07.186-.115c-.07-.04-.115-.096-.115-.096s.125-.144.28-.144zm-1.461.693c.156 0 .281.145.281.145s-.045.05-.115.095c.11.046.185.116.185.116s-.075.07-.185.115c.07.04.115.095.115.095s-.125.147-.281.147c-.156 0-.281-.147-.281-.147s.045-.055.115-.095a.674.674 0 0 1-.186-.115s.075-.07.186-.116c-.07-.04-.115-.095-.115-.095s.125-.145.28-.145zm-9.9.05-.924.497.048.096c.09.17.137.352.141.533l.98-.53a2.125 2.125 0 0 0-.197-.501zm11.236.948c.156 0 .281.147.281.147s-.045.055-.115.095c.11.046.185.116.185.116s-.075.07-.185.115c.07.04.115.095.115.095s-.13.147-.281.147c-.156 0-.281-.147-.281-.147s.044-.055.115-.095a.674.674 0 0 1-.186-.115s.075-.07.186-.116c-.07-.04-.115-.095-.115-.095s.125-.147.28-.147zm-2.844.399c.156 0 .281.144.281.144s-.045.056-.115.096c.11.045.186.115.186.115s-.075.07-.186.115c.07.04.115.096.115.096s-.125.146-.281.146c-.156 0-.281-.146-.281-.146s.045-.056.115-.096c-.11-.045-.186-.115-.186-.115s.076-.07.186-.115c-.07-.04-.115-.096-.115-.096s.125-.144.281-.144zm7.807.18c-.015.004-.03.015-.045.025l.004-.004-2.711.972.326.866c.508-.13 1.286-.69 1.834-1.121a.94.94 0 0 0 .045.453L24 7.617a.932.932 0 0 0-1.195-.547zm-6.37.255c.156 0 .282.147.282.147s-.045.055-.115.095a.588.588 0 0 1 .185.116s-.075.07-.185.115c.07.04.115.095.115.095s-.126.147-.281.147c-.156 0-.282-.147-.282-.147s.045-.055.116-.095c-.111-.045-.186-.115-.186-.115s.075-.07.186-.116c-.07-.04-.116-.095-.116-.095s.126-.147.282-.147zm1.778.559c.156 0 .281.144.281.144s-.045.056-.115.096a.56.56 0 0 1 .185.115s-.075.072-.185.117a.47.47 0 0 1 .115.094s-.125.147-.281.147c-.156 0-.281-.147-.281-.147s.045-.053.115-.094c-.11-.045-.186-.117-.186-.117s.075-.07.186-.115c-.07-.04-.115-.096-.115-.096s.125-.144.28-.144zm-3.887.386c.156 0 .281.145.281.145s-.044.055-.115.096c.11.045.186.115.186.115s-.075.072-.186.117a.47.47 0 0 1 .115.094s-.125.146-.28.146c-.157 0-.282-.146-.282-.146s.045-.054.115-.094c-.11-.045-.185-.117-.185-.117s.075-.07.185-.115c-.07-.04-.115-.096-.115-.096s.125-.145.281-.145zm-1.904.25c.156 0 .281.147.281.147s-.045.055-.115.096c.11.045.185.115.185.115s-.075.07-.185.115c.07.04.115.096.115.096s-.125.146-.281.146c-.156 0-.281-.146-.281-.146s.045-.05.115-.096a.674.674 0 0 1-.186-.115s.075-.07.186-.115c-.07-.04-.115-.096-.115-.096s.125-.147.28-.147zm3.611.342c.156 0 .281.147.281.147s-.044.055-.115.095c.11.04.186.116.186.116s-.075.07-.186.115c.07.04.115.096.115.096s-.125.146-.28.146c-.157 0-.282-.146-.282-.146s.045-.056.115-.096c-.11-.045-.185-.115-.185-.115s.075-.07.185-.116c-.07-.04-.115-.095-.115-.095s.125-.147.281-.147zm-2.703.85c.156 0 .281.146.281.146s-.045.054-.115.094c.11.04.186.117.186.117s-.075.07-.186.116c.07.04.115.095.115.095s-.125.145-.28.145c-.157 0-.282-.145-.282-.145s.047-.055.117-.095c-.11-.046-.187-.116-.187-.116s.077-.072.187-.117a.506.506 0 0 1-.117-.094s.125-.146.281-.146zm-2.27.02c.156 0 .282.146.282.146s-.045.055-.115.096c.11.045.185.115.185.115s-.075.07-.185.115c.07.04.115.096.115.096s-.126.144-.281.144c-.156 0-.282-.144-.282-.144s.045-.056.116-.096c-.111-.045-.186-.115-.186-.115s.075-.07.186-.115c-.07-.04-.116-.096-.116-.096s.126-.147.282-.147zm3.758.306c.156 0 .282.146.282.146s-.045.056-.116.096c.11.045.186.115.186.115s-.075.07-.186.116c.07.04.116.095.116.095s-.126.145-.282.145c-.155 0-.28-.145-.28-.145s.044-.055.114-.095a.688.688 0 0 1-.185-.116s.075-.07.185-.115c-.07-.04-.115-.096-.115-.096s.126-.146.281-.146zm-5.662.322c.156 0 .281.145.281.145s-.044.055-.115.096c.11.04.186.115.186.115s-.075.07-.186.115c.07.04.116.096.116.096s-.126.146-.282.146c-.155 0-.281-.146-.281-.146s.045-.056.115-.096c-.11-.045-.185-.115-.185-.115s.075-.07.185-.115c-.07-.04-.115-.096-.115-.096s.126-.145.281-.145zm3.356.803c.155 0 .281.147.281.147s-.045.055-.115.095a.56.56 0 0 1 .185.115s-.075.07-.185.116c.07.04.115.095.115.095s-.126.145-.281.145c-.156 0-.282-.145-.282-.145s.045-.055.116-.095c-.11-.046-.186-.116-.186-.116s.075-.07.186-.115c-.07-.04-.116-.095-.116-.095s.126-.147.282-.147zm-1.848.262c.156 0 .281.146.281.146s-.045.054-.115.094a.49.49 0 0 1 .186.117s-.075.07-.186.115c.07.04.115.096.115.096s-.125.145-.28.145c-.157 0-.282-.145-.282-.145s.045-.055.115-.096c-.11-.045-.185-.115-.185-.115s.075-.072.185-.117c-.07-.04-.115-.094-.115-.094s.125-.146.281-.146zm-3.316.08c.155 0 .28.146.28.146s-.044.056-.114.096c.11.045.185.115.185.115s-.075.07-.185.115c.07.04.115.096.115.096s-.126.145-.281.145c-.156 0-.282-.145-.282-.145s.047-.055.118-.096a.72.72 0 0 1-.188-.115s.077-.07.188-.115c-.07-.04-.118-.096-.118-.096s.126-.146.282-.146zm16.646.527-5.705 1.428-2.23 2.53-3.25.08-1.227 2.814-3.193.834-2 1.824h16.093c.834 0 1.512-.68 1.512-1.514zm-15.053.217c.156 0 .282.144.282.144s-.045.056-.116.096c.11.04.186.115.186.115s-.075.07-.186.116c.07.04.116.095.116.095s-.126.147-.282.147c-.155 0-.28-.147-.28-.147s.044-.055.114-.095a.688.688 0 0 1-.185-.116s.075-.07.185-.115c-.07-.04-.115-.096-.115-.096s.126-.144.281-.144zm-1.898 1.014c.156 0 .281.146.281.146s-.047.056-.117.096a.72.72 0 0 1 .188.115s-.077.07-.188.115c.07.04.117.096.117.096s-.13.15-.281.145c-.156 0-.281-.145-.281-.145s.045-.056.115-.096c-.11-.045-.186-.115-.186-.115s.075-.07.186-.115c-.07-.04-.115-.096-.115-.096s.125-.146.281-.146zm1.582.552c.156 0 .281.147.281.147s-.045.055-.115.096c.11.045.186.115.186.115s-.075.07-.186.115c.07.04.115.096.115.096s-.125.144-.281.144c-.156 0-.281-.144-.281-.144s.045-.056.115-.096c-.11-.045-.186-.115-.186-.115s.075-.07.186-.115c-.07-.04-.115-.096-.115-.096s.125-.147.281-.147zm-1.086 1.256c.156 0 .281.147.281.147s-.045.055-.115.095c.11.04.186.116.186.116s-.075.07-.186.115c.07.04.115.096.115.096s-.125.144-.28.144c-.157 0-.282-.144-.282-.144s.045-.056.115-.096c-.11-.045-.186-.115-.186-.115s.076-.07.186-.116c-.07-.04-.115-.095-.115-.095s.125-.147.281-.147zm-3.203 1.155-2.709 2.325a.969.969 0 0 0-.205-.473L0 19.277a.928.928 0 0 0 1.195.192l2.649-1.201z"
        /> < title > { title } < / title > < / svg >
    }
}
