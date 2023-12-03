# icp_rust_message_board_contract

### Requirements
* rustc 1.64 or higher
```bash
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ source "$HOME/.cargo/env"
```
* rust wasm32-unknown-unknown target
```bash
$ rustup target add wasm32-unknown-unknown
```
* candid-extractor
```bash
$ cargo install candid-extractor
```
* install `dfx`
```bash
$ DFX_VERSION=0.15.0 sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
$ echo 'export PATH="$PATH:$HOME/bin"' >> "$HOME/.bashrc"
$ source ~/.bashrc
$ dfx start --background
```

If you want to start working on your project right away, you might want to try the following commands:

```bash
$ cd icp_rust_boilerplate/
$ dfx help
$ dfx canister --help
```

## Update dependencies

update the `dependencies` block in `/src/{canister_name}/Cargo.toml`:
```
[dependencies]
candid = "0.9.9"
ic-cdk = "0.11.1"
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
ic-stable-structures = { git = "https://github.com/lwshang/stable-structures.git", branch = "lwshang/update_cdk"}
```

## did autogenerate

Add this script to the root directory of the project:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh
```

Update line 16 with the name of your canister:
```
https://github.com/buildwithjuno/juno/blob/main/scripts/did.sh#L16
```

After this run this script to generate Candid.
Important note!

You should run this script each time you modify/add/remove exported functions of the canister.
Otherwise, you'll have to modify the candid file manually.

Also, you can add package json with this content:
```
{
    "scripts": {
        "generate": "./did.sh && dfx generate",
        "gen-deploy": "./did.sh && dfx generate && dfx deploy -y"
      }
}
```

and use commands `npm run generate` to generate candid or `npm run gen-deploy` to generate candid and to deploy a canister.

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
$ dfx start --background

# Deploys your canisters to the replica and generates your candid interface
$ dfx deploy
```






//////////////

Hello Dunstan,

I trust you're doing well. I wanted to inform you that I've just submitted a pull request to your repository. The changes I've proposed are aimed at enhancing the project's functionality and addressing specific issues.

<details>
  <summary>Click to show details</summary>

# Bugs Fixing, Error Handling And Code Improvement.
  
1) - add_food_item

Suggestion:
Instead of using expect which will panic on error, use unwrap_or or unwrap_or_else to provide a default value when incrementing the FOOD_ID_COUNTER.


Why:
Using expect for error handling can lead to panics, which can crash the program. It is better to handle the error gracefully by providing a default value using unwrap_or or unwrap_or_else.



Suggestion:
The do_insert_food_item function should be inlined into add_food_item function to reduce unnecessary function call overhead.


Why:
Inlining the do_insert_food_item function into add_food_item eliminates the overhead of a function call. This can improve the performance of the code by reducing the time spent on function call setup and teardown.

Suggestion:
Use a constant for the expiration date offset 86_400_000_000_000 to improve readability.


Why:
Using a constant for the expiration date offset improves readability by providing a descriptive name for the value. It makes the code more self-explanatory and easier to understand.


2) update_food_item-----------------

Suggestion:
Change FOOD_STORAGE.with(|service| service.borrow().get(&id)) in the code to _get_food_item(&id) to reuse the existing function and improve code readability.


Why:
By reusing the existing _get_food_item function instead of directly accessing FOOD_STORAGE, the code becomes more readable and easier to understand. It also promotes code reuse and reduces duplication.


Suggestion:
The code uses do_insert_food_item function to update the food_item. Instead, it should directly use FOOD_STORAGE.with(|service| service.borrow_mut().insert(food_item.id, food_item.clone())) to avoid unnecessary function call.


Why:
The suggestion is important because it eliminates the need for an additional function call, improving performance and reducing unnecessary code complexity.


Suggestion:
The code uses format! function to generate error message. Instead, it should use a constant string with placeholders to avoid unnecessary string formatting operations.


Why:
Using a constant string with placeholders for error message improves performance by avoiding unnecessary string formatting operations. It also follows the best practice of separating the error message from the formatting operation.


3) delete_food_item-------------------------


Suggestion:
The code does not log any information about the operation. Instead, it should use the ic_cdk::println function to log information about the operation for debugging purposes.


Why:
Logging information about the operation can be helpful for debugging and troubleshooting purposes. It allows developers to track the flow of the code and identify any issues or unexpected behavior.

Suggestion:
The code directly returns the removed FoodItem. Instead, it should return a success message or status code to indicate that the operation was successful.


Why:
Returning a success message or status code instead of the removed FoodItem provides a clearer indication of the operation's success. It separates the concerns of deleting the item and returning its details, making the code more modular and easier to understand.


Suggestion:
The code directly uses the FOOD_STORAGE thread-local variable. Instead, it should abstract the storage operations into a separate FoodStorage struct and use methods on that struct to perform operations.


Why:
Abstracting the storage operations into a separate struct improves code organization and maintainability. It encapsulates the storage logic and provides a clear interface for performing operations on the storage. This separation of concerns makes the code easier to understand, test, and modify in the future.


Suggestion:
The code uses format! for the error message. Instead, it should use a static string for the error message and include the id as a separate field in the Error enum.


Why:
Including the id as a separate field in the Error enum allows for better error handling and provides more information about the error. Using a static string for the error message ensures consistency and avoids unnecessary formatting.


4) check_expiration_status--------------

Suggestion:
Consider using if let instead of match for handling the Option returned by _get_food_item to make the code more concise.


Why:
Using if let instead of match can make the code more concise and easier to read by eliminating the need for a match expression and reducing the amount of code.

Suggestion:
Instead of returning a String for the expiration status, consider using an enum with Expired and NotExpired variants. This makes the return type more expressive and less error-prone.


Why:
Using an enum for the expiration status provides a more expressive and type-safe way of representing the possible values. It eliminates the possibility of returning an invalid or misspelled string, reducing the chance of runtime errors.


5) list_all_food_items------------

suggestion:
Change service.borrow_mut() to service.borrow() because the function does not mutate the FOOD_STORAGE.


Why:
The suggestion is important because it improves code readability and avoids unnecessary mutability. Since the list_all_food_items function does not mutate the FOOD_STORAGE, there is no need to use borrow_mut(). Using borrow() instead clearly communicates the intent of the code and prevents accidental mutations.


6) get_total_food_quantity-------

Suggestion:
Change service.borrow_mut() to service.borrow() because the function does not mutate the data.


Why:
The suggestion is important because the get_total_food_quantity function does not modify the data in the FOOD_STORAGE. Therefore, there is no need to use borrow_mut() which acquires a mutable borrow. Using borrow() instead will improve performance and adhere to the best practice of using the least restrictive borrow method.

Suggestion:
Consider adding error handling for potential panics when borrowing the RefCell.


Why:
Error handling is important to handle potential panics that may occur when borrowing the RefCell. Panics can occur if there are multiple mutable borrows of the RefCell at the same time, which violates Rust's borrowing rules. By adding error handling, we can gracefully handle these panics and provide a more robust and reliable implementation.

Suggestion:
Consider adding logging for debugging and tracking purposes.


Why:
Adding logging to the code can provide valuable information for debugging and tracking purposes. It allows developers to trace the execution flow, identify potential issues, and monitor the behavior of the function.

</details>
