pub struct Options {
    pub message: String,
    pub repo: String,
}
pub struct Interface {
    opts: Options,
    repo: git2::Repository,
    author: String,
    pub secret: String,
}
impl Interface {
    pub fn new(opts: Options) -> Result<Interface, &'static str> {
        let repo = match git2::Repository::open(&opts.repo) {
            Ok(r) => r,
            Err(_) => {
                return Err("Failed to open repository");
            }
        };
        let author = Interface::load_author(&repo)?;
        let secret = Default::default();

        Ok(Interface {
            opts,
            repo,
            author,
            secret,
        })
    }

    fn load_author(repo: &git2::Repository) -> Result<String, &'static str> {
        let cfg = match repo.config() {
            Ok(c) => c,
            Err(_) => {
                return Err("Failed to load git config user.name");
            }
        };

        let name = match cfg.get_string("user.name") {
            Ok(s) => s,
            Err(_) => {
                return Err("Failed to find git config user.name");
            }
        };

        let email = match cfg.get_string("user.email") {
            Ok(s) => s,
            Err(_) => {
                return Err("Failed to find git config user.email");
            }
        };

        Ok(format!("{} <{}>", name, email))
    } //end load_author
} //end Impl Interface
