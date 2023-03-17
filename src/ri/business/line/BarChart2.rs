#[cfg(feature = "RiBusinessLineBarChart2")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessLineBarChart2")]
/// *This icon requires the feature* `RiBusinessLineBarChart2` *to be enabled*.
#[component]
pub fn BarChart2(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M2 13h6v8H2v-8zm14-5h6v13h-6V8zM9 3h6v18H9V3zM4 15v4h2v-4H4zm7-10v14h2V5h-2zm7 5v9h2v-9h-2z" /></g></svg>
   }
}