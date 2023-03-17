#[cfg(feature = "RiBusinessLineHonour")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "RiBusinessLineHonour")]
/// *This icon requires the feature* `RiBusinessLineHonour` *to be enabled*.
#[component]
pub fn Honour(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><g><path fill="none" d="M0 0h24v24H0z" /><path d="M21 4v14.721a.5.5 0 0 1-.298.458L12 23.03 3.298 19.18A.5.5 0 0 1 3 18.72V4H1V2h22v2h-2zM5 4v13.745l7 3.1 7-3.1V4H5zm3 4h8v2H8V8zm0 4h8v2H8v-2z" /></g></svg>
   }
}