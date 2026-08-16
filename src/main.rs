use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="site">

            // ── Navigation ──────────────────────────────────────
            <nav class="nav">
                <a class="logo" href="#">{ "uttkarzsh" }</a>

                <div class="nav-links">
                    <a href="#work">{ "work" }</a>
                    <a href="#about">{ "about" }</a>
                    <a href="#contact">{ "contact" }</a>
                </div>
            </nav>


            // ── Hero ────────────────────────────────────────────
            <main>

                <section class="hero">
                    <p class="eyebrow">
                        { "coder | larper | engineering student" }
                    </p>

                    <h1>
                        { "welcome to my" }
                        <span>{ " roman empire" }</span>
                    </h1>

                    <p class="hero-description"><i>
                        { "to make an apple pie from scratch, you must first invent the universe" }
                    </i></p>

                    <div class="socials">
                        <a
                            href="https://github.com/uttkarzsh"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            { "github ↗" }
                        </a>

                        <a
                            href="https://x.com/uttkarzsh"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            { "twitter/x ↗" }
                        </a>
                        <a
                            href="https://linkedin.com/in/uttakrsh-pawan"
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            { "linkedin ↗" }
                        </a>
                    </div>
                </section>


                // ── Work ────────────────────────────────────────
                <section id="work" class="section">

                    <div class="section-label">
                        <span>{ "01" }</span>
                        <h2>{ "selected work" }</h2>
                    </div>

                    <div class="projects">

                        <Project
                            number="01"
                            title="RiasOS"
                            description="A tiny 64-bit operating system written from scratch in Rust."
                            tags="Rust · x86_64 · OSDev"
                        />

                        <Project
                            number="02"
                            title="EulaLang"
                            description="an LLVM based compiler"
                            tags="C++ · LLVM"
                        />

                        <Project
                            number="03"
                            title="Groth16"
                            description="A Rust implementation of the Groth16 proving system"
                            tags="Rust · Elliptic Curve Cryptography · Arkworks"
                        />

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
                            { "hi. i'm uttkarsh, an undergrad student from iit roorkee. \
                            i work mostly with rust and sometimes c++." }
                        </p>

                        <p>
                            { "currently exploring kernels & operating systems, compiler design \
                              and distributed systems. i also like ai and have a bit of experience \
                              working with zero-knowledge proving systems and web3 in general." }
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
                        { "Let's build something interesting." }
                    </h2>

                    <a
                        class="contact-link"
                        href="mailto:utkbest@gmail.com"
                    >
                        { "email ↗" }
                    </a>
                </section>

            </main>


            // ── Footer ──────────────────────────────────────────
            <footer>
                <span>{ "entropy finds everyone" }</span>
                <span>{ "built with yew.rs" }</span>
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
}

#[function_component(Project)]
fn project(props: &ProjectProps) -> Html {
    html! {
        <a class="project" href="#">

            <div class="project-info">

                <span class="project-number">
                    { &props.number }
                </span>

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
