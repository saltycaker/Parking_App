use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

mod components;
mod pages;
mod services;
mod types;

use components::layout::Layout;
use pages::home::Home;
use pages::parking_detail::ParkingDetail;
use pages::navigation::Navigation;
use pages::profile::Profile;

#[component]
pub fn App() -> impl IntoView {
    // Provides context for the entire app
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/style/output.css"/>
        
        <Title text="Parking Discovery"/>
        
        <Router>
            <main class="min-h-screen bg-white">
                <Layout>
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/parking/:id" view=ParkingDetail/>
                        <Route path="/navigation/:id" view=Navigation/>
                        <Route path="/profile" view=Profile/>
                    </Routes>
                </Layout>
            </main>
        </Router>
    }
}
