#[cfg(feature = "BiSolidCastle")]
use leptos::{component, Scope, IntoView, view};

#[cfg(feature = "BiSolidCastle")]
/// *This icon requires the feature* `BiSolidCastle` *to be enabled*.
#[component]
pub fn Castle(cx: Scope) -> impl IntoView {
   view! { cx,
       <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24"><path d="M20 11h-2V6h1V2h-2v2h-1V2h-2v2h-1V2h-2v2h-1V2H8v2H7V2H5v4h1v5H4V9H2v12h7v-5a3 3 0 0 1 6 0v5h7V9h-2zm-10-1H8V7h2zm6 0h-2V7h2z" /></svg>
   }
}