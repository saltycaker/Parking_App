use leptos::*;
use leptos_router::*;

#[component]
pub fn Profile() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-white">
            <div class="max-w-2xl mx-auto p-4">
                // Header
                <div class="flex items-center justify-between mb-6">
                    <A href="/" class="inline-flex items-center text-gray-600 hover:text-gray-900">
                        <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
                        </svg>
                        "Back"
                    </A>
                    <h1 class="text-2xl font-bold text-gray-900">Profile</h1>
                    <div class="w-16"></div>
                </div>

                // User Info
                <div class="flex items-center gap-4 mb-8">
                    <div class="w-20 h-20 bg-gray-100 rounded-full flex items-center justify-center">
                        <svg class="w-10 h-10 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"/>
                        </svg>
                    </div>
                    <div class="flex-1">
                        <h2 class="text-2xl font-bold text-gray-900">John Doe</h2>
                        <p class="text-gray-600">john.doe@example.com</p>
                    </div>
                    <button class="text-purple-600 font-semibold">Edit</button>
                </div>

                // Saved Locations
                <div class="mb-8">
                    <h3 class="text-lg font-bold text-gray-900 mb-4">Saved Locations</h3>
                    <div class="space-y-3">
                        <div class="bg-gray-50 rounded-xl p-4 flex items-center gap-3">
                            <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
                            </svg>
                            <div class="flex-1">
                                <p class="font-semibold text-gray-900">Home</p>
                                <p class="text-sm text-gray-600">123 Home Street</p>
                            </div>
                        </div>
                        <div class="bg-gray-50 rounded-xl p-4 flex items-center gap-3">
                            <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 13.255A23.931 23.931 0 0112 15c-3.183 0-6.22-.62-9-1.745M16 6V4a2 2 0 00-2-2h-4a2 2 0 00-2 2v2m4 6h.01M5 20h14a2 2 0 002-2V8a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"/>
                            </svg>
                            <div class="flex-1">
                                <p class="font-semibold text-gray-900">Work</p>
                                <p class="text-sm text-gray-600">456 Office Avenue</p>
                            </div>
                        </div>
                    </div>
                </div>

                // Favorites
                <div class="mb-8">
                    <h3 class="text-lg font-bold text-gray-900 mb-4">Favorite Parking</h3>
                    <div class="space-y-3">
                        <div class="bg-gray-50 rounded-xl p-4 flex items-center gap-3">
                            <svg class="w-6 h-6 text-red-500" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M4.318 6.318a4.5 4.5 0 000 6.364L12 20.364l7.682-7.682a4.5 4.5 0 00-6.364-6.364L12 7.636l-1.318-1.318a4.5 4.5 0 00-6.364 0z"/>
                            </svg>
                            <div class="flex-1">
                                <p class="font-semibold text-gray-900">Central Parking Garage</p>
                                <p class="text-sm text-gray-600">123 Main Street</p>
                            </div>
                        </div>
                    </div>
                </div>

                // Settings
                <div class="mb-8">
                    <h3 class="text-lg font-bold text-gray-900 mb-4">Settings</h3>
                    <div class="space-y-3">
                        <div class="bg-gray-50 rounded-xl p-4 flex items-center justify-between">
                            <div class="flex items-center gap-3">
                                <svg class="w-6 h-6 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"/>
                                </svg>
                                <span class="text-gray-900">Notifications</span>
                            </div>
                            <div class="w-12 h-6 bg-purple-600 rounded-full relative">
                                <div class="absolute right-1 top-1 w-4 h-4 bg-white rounded-full"></div>
                            </div>
                        </div>
                        <div class="bg-gray-50 rounded-xl p-4 flex items-center justify-between">
                            <div class="flex items-center gap-3">
                                <svg class="w-6 h-6 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"/>
                                </svg>
                                <span class="text-gray-900">Dark Mode</span>
                            </div>
                            <div class="w-12 h-6 bg-gray-300 rounded-full relative">
                                <div class="absolute left-1 top-1 w-4 h-4 bg-white rounded-full"></div>
                            </div>
                        </div>
                    </div>
                </div>

                // Logout
                <button class="w-full border-2 border-red-600 text-red-600 py-4 rounded-xl font-semibold hover:bg-red-50 transition">
                    "Log Out"
                </button>
            </div>
        </div>
    }
}
