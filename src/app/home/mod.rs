use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {
    view! { cx,
        <div id="title">
            <h1>"Intellegently Manage Donations"</h1>
        </div>

        <div>
            <h2>"What Is The Engine"</h2>
            <h4>
                "Donation Engine is a project to allow for more intellegent donation lists.
                If you create a list to manage your donations you might have an idea of how
                much you value each charity, and how much you would like to donate to it.
                However, unless you setup an automated spreadsheet, you have to manually
                come up with the amount to donate to each charity."
            </h4>
        </div>

        <div>
            <h2>"Donation Categories"</h2>
            <h4>
                "Donation Engine organizes donations by group. Let's say you would like to
                donate to a number of global heath charities. You can place all of them in a
                Gobal Health group."
            </h4>
        </div>

        <div>
            <h2>"Value Controls"</h2>
            <h4>
                "Donation value for categories or charities can be contorlled by a value
                multiplier, a percentage, or a donation amount. Group global health can
                receive 40% of your total donation value, and an entry in the group can be
                valued at twice the amount as any other, while another can be fixed to $200."
            </h4>
        </div>

        <div>
            <h2>"Auto Weighting"</h2>
            <h4>
                "Use charity ratings from Charity Navigator to auto weight charities in a
                group. This lets you value higher rated charites more by just clicking a
                button."
            </h4>
        </div>

        <div>
            <h2>"Every.org"</h2>
            <h4>
                "Our site links directly to Every.org, so you can donated at your computed
                amounts easily. Every.org partners with Network For Good to distribute your
                donations to your charities with the option to keep your donation anonymous."
            </h4>
        </div>

        <div>
            <h2>Presets</h2>
            <h4>
                Import curated presets for standard categories like global health. Also,
                publish a group of your own and search categories made by others.
            </h4>
        </div>
    }
}
