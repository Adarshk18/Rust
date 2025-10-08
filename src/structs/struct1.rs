struct User{
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64,
}



fn main(){
	let user1 = User{
		active: true,
		username: String::from("adarsh12"),
		email: String::from("adarsh@gmail.com"),
		sign_in_count: 1,
	};

	//if user1 is mutable we can change the value of its fields
	let mut user2 = User{
		active: true,
		username: String::from("adarsh1"),
		email:String::from("adarsh1@gmail.com"),
		sign_in_count: 2,
	};

	user1.email = String::from("adarsh3@gmail.com");
}

fn build_user(username: String, email: String) -> User{
	User{
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}


