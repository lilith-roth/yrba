use console::{Emoji, style};

pub(crate) fn write_welcome_message() {
    let intro_logo: &str = r#"
        
        ▖▖▄▖▄ ▄▖                                                
        ▌▌▙▘▙▘▌▌                                                
        ▐ ▌▌▙▘▛▌                                                
                                                                
        ▖▖        ▄▖       ▗     ▄     ▌       ▄▖    ▘  ▗     ▗ 
        ▌▌▛▌▌▌▛▘  ▙▘█▌▛▛▌▛▌▜▘█▌  ▙▘▀▌▛▘▙▘▌▌▛▌  ▌▌▛▘▛▘▌▛▘▜▘▀▌▛▌▜▘
        ▐ ▙▌▙▌▌   ▌▌▙▖▌▌▌▙▌▐▖▙▖  ▙▘█▌▙▖▛▖▙▌▙▌  ▛▌▄▌▄▌▌▄▌▐▖█▌▌▌▐▖

        "#;
    log::info!(
        "{}\n\t\t{}{}\n\n",
        style(intro_logo).green(),
        style("Trans rights are human rights!").magenta().bright(),
        Emoji("⚧️ 💜", "")
    );
}
