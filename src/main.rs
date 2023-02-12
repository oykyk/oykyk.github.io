use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html!{
        <>
            
            <div class="top-container">
                <img class="TopCloud" src="images/cloud.png" alt="cloud-img"/>
                <h1>{"I'm yk."}</h1>
                <h2>{"a "} <span class="pro">{"pro"}</span>{"grammer"}</h2>
                <img class="BottomCloud" src="images/cloud.png" alt="cloud-img"/>
                <img src="images/mountain.png" alt="mountain-img"/>
            </div>

            <div class="middle-container">
                <div class="profile">
                    <img class="boy-img" src="images/ykyk.png" alt="yk-img"/>
                    <h2>{"Hello."}</h2>
                    <p class="intro">{"I'm a Software Developer working in the cybersecurity industry 😎"}</p>
                </div>
            
                <hr/>

                <div class="skills">
                    <h2>{"My Skills."}</h2>
        
                    <div class="skill-row">
                        <img class="code-img" src="images/computer.png" alt="code-img"/>
                        <h3>{"Design & Development"}</h3>
                        <p>{"I love building software tools in C and Rust!"}</p>
                    </div>
        
                    <div class="skill-row">
                        <img class="chilli-img" src="images/chillies.png" alt="chilli-img"/>
                        <h3>{"Wasm development"}</h3>
                        <p>{"Right now I'm very interested in Rust & Wasm programming language. Did you know this profile page is built using Rust webassemnly framework? Check out my github to learn more!"}</p>
                    </div>
                </div>

                <hr/>
                
                <div class="contact-me">
                    <h2>{"Get In Touch!"}</h2>
                    <h3>{"If you love programming as much as I do."}</h3>
                    <p class="contact-message">{"Love programming as much as I do? Say hello and we can talk code all day!"}</p>
                    <a class="btn" href="mailto:name@email.com">{"CONTACT ME"}</a>
                </div>
            </div>

            <div class="bottom-container">
                <a class="footer-link" href="https://www.linkedin.com/">{"LinkedIn"}</a>
                <a class="footer-link" href="https://twitter.com/">{"Twitter"}</a>
                <a class="footer-link" href="https://github.com/">{"Github"}</a>
                <p class="copyright-description">{"© 2023 YK."}</p>
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}