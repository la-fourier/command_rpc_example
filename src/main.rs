use command_rpc::crpc_main;

#[crpc_main]
pub mod database {

    use command_rpc::{crpc_mod, crpc_fn};

    #[crpc_fn]
    pub fn connect(
        /// This is the name of the database
        name: String,
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        password: String,
    ) {
        println!("Connect to database {:?} with user {:?}", name, user);
    }

    #[crpc_fn]
    pub fn disconnect(name: String) {
        println!("Disconnect from database {:?}", name);
    }

    #[crpc_mod]
    /// Here you can modify the user
    pub mod user {
        use command_rpc::crpc_fn;

        #[crpc_fn]
        pub fn create_user(
            /// This is the name of the user
            name: String,
            email: String,
            #[arg(short, long)]
            birth_date: String,
            #[arg(default_value = "0")]
            access_level: i32,
        ) {
            println!("Create user {:?} with mail {:?}", name, email);
        }

        #[crpc_fn]
        pub fn update_user(name: String, email: String) {
            println!("Update user {:?} with mail {:?}", name, email);
        }

        #[crpc_fn]
        pub fn delete_user(name: String, email: String) {
            println!("Delete user {:?} with mail {:?}", name, email);
        }

        #[crpc_fn]
        pub fn read_user(name: String, email: String) {
            println!("Read user {:?} with mail {:?}", name, email);
        }
    }

}

fn main() {
    let args = Database::parse();
    args.delegate();
}