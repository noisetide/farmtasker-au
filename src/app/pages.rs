use super::products::CfgProductItemsList;
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="main_buttons_online_shops_container">
            <a href="/shop/food" class="page-selector-container" id="button_farm_to_table_near_me">
                <img class="page-selector-image" src="/main_buttons/food_shop.png" alt="Online Shop"/>
            </a>
            <a href="/shop/pet" class="page-selector-container" id="button_farmtasker_pet_food_shop">
                <img class="page-selector-image" src="/main_buttons/pet_food_shop.png" alt="Online Shop"/>
            </a>
        </div>
        <div class="main_buttons_services_container">
            <a href="/shop/eat" class="page-selector-container" id="button_ready_to_eat_shop">
                <img class="page-selector-image" src="/main_buttons/ready_to_eat_shop.png" alt="Eat Now"/>
            </a>
            <a href="/instructions" class="page-selector-container" id="button_farm_task_video_instructions_service">
                <img class="page-selector-image" src="/main_buttons/instructions.png" alt="Video Instructions"/>
            </a>
            <a href="/blog/culinary-adventure" class="page-selector-container" id="button_culinary_adventure">
                <img class="page-selector-image" src="/main_buttons/video_blog.png" alt="Video Blog"/>
            </a>
        </div>
    }
}

#[component]
pub fn PetFood() -> impl IntoView {
    view! {
        <CfgProductItemsList items_category="pet_food".to_string()/>
    }
}

#[component]
pub fn FarmFood() -> impl IntoView {
    view! {
        <CfgProductItemsList items_category="food".to_string()/>
    }
}

#[component]
pub fn ReadyToEat() -> impl IntoView {
    view! {
        <CfgProductItemsList items_category="ready_to_eat".to_string()/>
    }
}

#[component]
pub fn VideoInstructions() -> impl IntoView {
    view! {
        <div class="blog-container">
            <img class="banner-image" src="/banners/instructions.webp" alt="Video Instructions Banner"/>
            <iframe
                class="embed-video"
                src="https://www.youtube.com/embed/daOJwUdwTME?si=GrQjf7z3ZiHD4WQF"
                title="Instructions"
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                referrerpolicy="strict-origin-when-cross-origin"
                allowfullscreen
            ></iframe>
        </div>
    }
}

#[component]
pub fn VideoBlogs() -> impl IntoView {
    view! {
        <div class="blog-container">
            <img class="banner-image" src="/banners/video_blog.webp" alt="Video Blog Banner"/>
            <iframe
                class="embed-video"
                src="https://www.youtube.com/embed/EFyeoMRsDN8?si=pqqFHuqhTuB5xNMV"
                title="Culinary Adventure"
                frameborder="0"
                allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share"
                referrerpolicy="strict-origin-when-cross-origin"
                allowfullscreen
            ></iframe>
        </div>
        <a href="/instructions" class="page-selector-container" id="button_farm_task_video_instructions">
            <img style="max-height: 20rem" class="page-selector-image" src="/main_buttons/instructions.png" alt="Video Instructions"/>
        </a>
    }
}

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class="blog-container">
            <img class="banner-image" src="/banners/about_us_cropped.webp" alt="About Us Banner"/>
            <img class="banner-photo" src="/photos/about_us_group_photo.webp" alt="About Us Photo Banner"/>
        </div>
        <div class="about-us-blocks-container">
            <ul class="about-us-block">
                <h3>"Olesia – Director & Co-Founder (50% Farmtasker PTY LTD)"</h3>
                <h4>"With over 25 years of business and finance experience in Ukraine, Olesia is the driving force behind Farmtasker. She successfully launched her first online shop in 2004, showcasing her entrepreneurial vision."</h4>
                    <h3>"Experience:"</h3>
                        <li style="font-size: 16px">"Bookkeeping in Australia: May 2022 – present"</li>
                        <li style="font-size: 16px">"Poultry processing expertise: December 2022 – present"</li>
                    <h3>"Education:"</h3>
                        <li style="font-size: 16px">"Master's Degree in Economics (2001)"</li>
                <h4>"Olesia’s leadership combines strategic insight with hands-on industry knowledge, ensuring the smooth operation of Farmtasker."</h4>
            </ul>
            <ul class="about-us-block">
                <h3>"Vasyl – Sales & Marketing Manager, Co-Founder (50% Farmtasker PTY LTD)"</h3>
                <h4>"Vasyl brings a wealth of expertise with 10+ years of experience in government roles, including the State Tax Service of Ukraine and Legal Advisory for Kyiv Consulate. His qualifications are officially recognized by the Australian Government."</h4>
                    <h3>"Experience:"</h3>
                        <li style="font-size: 16px">"Sales & Business Management: Nearly 10 years of success in Ukraine"</li>
                        <li style="font-size: 16px">"Poultry processing expertise: December 2022 – present"</li>
                    <h3>"Education:"</h3>
                        <li style="font-size: 16px">"Master’s Degree in Finance (2007)"</li>
                        <li style="font-size: 16px">"Master’s Degree in Law (2011)"</li>
                <h4>"Vasyl’s strategic sales approach and legal experience ensure Farmtasker delivers with professionalism and integrity."</h4>
            </ul>
            <ul class="about-us-block">
                <h3>"Dmytro – Software Engineer, Farmtasker PTY LTD"</h3>
                <h4>"As the tech backbone of Farmtasker, Dmytro combines his technical prowess with creative skills."</h4>
                    <h3>"Experience:"</h3>
                        <li style="font-size: 16px">"1.8 years as a Multimedia Officer, Future Digital Department, University of Tasmania"</li>
                        <li style="font-size: 16px">"Kitchen hand experience: Agrarian Kitchen, New Norfolk (December 2023 – December 2024)"</li>
                        <li style="font-size: 16px">"5 years of sound engineering and video editing experience"</li>
                    <h3>"Education:"</h3>
                        <li style="font-size: 16px">"Dmytro is currently pursuing his Bachelor’s Degree in Software Engineering (2025), playing a key role in building Farmtasker’s online presence and digital solutions."</li>
            </ul>
            <ul class="about-us-block">
                <h3>"Margo – Co-Founders’ Daughter & Future Vet"</h3>
                    <h4>"Born in 2012, Margo embodies the family’s next generation of passion and energy. A devoted animal lover, she aspires to become a veterinarian while actively contributing to family projects."</h4>
            </ul>
        </div>
    }
}

#[component]
pub fn Delivery() -> impl IntoView {
    view! {
        // TODO
    }
}

#[component]
pub fn PrivacyPolicy() -> impl IntoView {
    view! {
        <div class="privacy-policy">
            <h1>"Privacy Policy"</h1>
            <p>
                "Our Privacy Policy is currently being prepared and will be available soon. If you have any questions, please contact us at "
                <a href="mailto:farmtasker@gmail.com">" farmtasker@gmail.com"</a>
                "."
            </p>
        </div>
    }
}

#[component]
pub fn TermsOfService() -> impl IntoView {
    view! {
        <div class="terms-of-service">
            <h1>"Terms of Service"</h1>
            <p>
                "Our Terms of Service are currently being prepared and will be available soon. If you have any questions, please contact us at "
                <a href="mailto:farmtasker@gmail.com">" farmtasker@gmail.com"</a>
                "."
            </p>
        </div>
    }
}
