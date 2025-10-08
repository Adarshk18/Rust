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
	// let mut user2 = User{
	// 	active: true,
	// 	username: String::from("adarsh1"),
	// 	email:String::from("adarsh1@gmail.com"),
	// 	sign_in_count: 2,
	// };

	// user1.email = String::from("adarsh3@gmail.com");

	// let user2 = User{
	// 	active: user1.active,
	// 	username: user1.username,
	// 	email: String::from("adarsg3@gmail.com"),
	// 	sign_in_count: user1.sign_in_count,
	// };

	//another way fo doing the above
	let user2 = User{
		email: String::from("adarsh654@gmail.com"),
		..user1
	};
}

fn build_user(username: String, email: String) -> User{
	User{
		active: true,
		username,
		email,
		sign_in_count: 1,
	}
}


