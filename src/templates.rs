use askama::Template;

#[derive(Template)]
#[template(path = "bash/init")]
pub struct Bash;
