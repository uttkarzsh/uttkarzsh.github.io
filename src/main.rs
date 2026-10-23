use yew::prelude::*;

// ─────────────────────────────────────────────────────────────────
// Content
// ─────────────────────────────────────────────────────────────────

const SOCIALS: &[(&str, &str)] = &[
    ("github", "https://github.com/uttkarzsh"),
    ("twitter/x", "https://x.com/uttkarzsh"),
    ("linkedin", "https://linkedin.com/in/uttkarsh-pawan"),
];

struct ProjectData {
    title: &'static str,
    description: &'static str,
    tags: &'static str,
    github: &'static str,
}

const PROJECTS: &[ProjectData] = &[
    ProjectData {
        title: "RiasOS",
        description: "The crimson-kerneled x86_64 operating system written from scratch in Rust.",
        tags: "Rust · x86-64 · Operating Systems",
        github: "https://github.com/uttkarzsh/rias",
    },
    ProjectData {
        title: "Laniakea",
        description: "A ZK proving SDK for iOS devices",
        tags: "Rust · Arkworks",
        github: "https://github.com/uttkarzsh/laniakea",
    },
    ProjectData {
        title: "Groth16",
        description: "A Rust implementation of the Groth16 proving system",
        tags: "Rust · Elliptic Curve Cryptography · Arkworks",
        github: "https://github.com/uttkarzsh/groth16",
    },
];

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="site">

            // ── Navigation ──────────────────────────────────────
            <nav class="nav">
                <a class="logo" href="#top">
                    <span class="logo-dot"></span>
                    { "uttkarzsh" }
                </a>

                <div class="nav-links">
                    <a href="#work">{ "work" }</a>
                    <a href="#about">{ "about" }</a>
                    <a href="#contact">{ "contact" }</a>
                </div>
            </nav>


            // ── Hero ────────────────────────────────────────────
            <main id="top">

                <section class="hero">
                    <p class="eyebrow reveal">
                        { "coder | systems | inference" }
                    </p>

                    <h1 class="reveal">
                        { "welcome to my" }
                        <span>{ " roman empire" }</span>
                    </h1>

                    <p class="hero-description reveal"><i>
                        { "to make an apple pie from scratch, you must first invent the universe" }
                    </i></p>

                    <div class="socials reveal">
                        { for SOCIALS.iter().map(|(label, href)| html! {
                            <a href={*href} target="_blank" rel="noopener noreferrer">
                                { *label }
                                <span class="arrow">{ "↗" }</span>
                            </a>
                        }) }
                    </div>

                    <figure class="art reveal">
                        <ascii-art
                            piece="cloud-sea"
                            label="the sun sinking into a drifting sea of orange cloud at dusk"
                        ></ascii-art>

                        <figcaption>
                            <span>{ "fig. 00" }</span>
                            <span>{ "cloud sea" }</span>
                        </figcaption>
                    </figure>
                </section>


                // ── Work ────────────────────────────────────────
                <section id="work" class="section">

                    <div class="section-label">
                        <span>{ "01" }</span>
                        <h2>{ "works" }</h2>
                    </div>

                    <div class="projects">
                        { for PROJECTS.iter().enumerate().map(|(i, p)| html! {
                            <Project
                                number={format!("{:02}", i + 1)}
                                title={p.title}
                                description={p.description}
                                tags={p.tags}
                                github={p.github}
                            />
                        }) }
                    </div>
                </section>


                // ── About ───────────────────────────────────────
                <section id="about" class="section about">

                    <div class="section-label">
                        <span>{ "02" }</span>
                        <h2>{ "about" }</h2>
                    </div>

                    <div class="about-content">
                        <p>
                            { "hi. i'm uttkarsh, an undergrad engineering student from iit roorkee. \
                            i work mostly with rust and sometimes c++ and python." }
                        </p>

                        <p>
                            { "currently exploring kernels & operating systems, compiler design & \
                               inference engineering. i also have a bit of experience working \
                               with zero-knowledge proving systems and web3 in general." }
                        </p>
                    </div>

                </section>


                // ── Contact ─────────────────────────────────────
                <section id="contact" class="section contact">

                    <div class="section-label">
                        <span>{ "03" }</span>
                        <h2>{ "contact" }</h2>
                    </div>

                    <h2 class="contact-heading">
                        { "Let's build something " }
                        <span>{ "interesting." }</span>
                    </h2>

                    <a
                        class="contact-link"
                        href="mailto:utkbest@gmail.com"
                    >
                        { "utkbest@gmail.com" }
                        <span class="arrow">{ "↗" }</span>
                    </a>
                </section>

            </main>


            // ── Footer ──────────────────────────────────────────
            <footer>
                <span>{ "entropy finds everyone" }</span>
                <a href="https://yew.rs" target="_blank" rel="noopener noreferrer">
                    { "built with yew.rs" }
                </a>
            </footer>

        </div>
    }
}

// ─────────────────────────────────────────────────────────────────
// Project component
// ─────────────────────────────────────────────────────────────────

#[derive(Properties, PartialEq)]
struct ProjectProps {
    number: AttrValue,
    title: AttrValue,
    description: AttrValue,
    tags: AttrValue,
    github: AttrValue,
}

#[function_component(Project)]
fn project(props: &ProjectProps) -> Html {
    html! {
        <a class="project" href={props.github.clone()} target="_blank" rel="noopener noreferrer">

            <span class="project-number">
                { &props.number }
            </span>

            <div class="project-info">

                <h3>
                    { &props.title }
                </h3>

                <p>
                    { &props.description }
                </p>

                <span class="project-tags">
                    { &props.tags }
                </span>

            </div>

            <span class="project-arrow">
                { "↗" }
            </span>

        </a>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
