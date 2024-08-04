use ai_functions::ai_function;

#[ai_function]
pub fn print_backend_webserver_code(_project_description_and_template: &str) {
    /// Input: Takes in a PROJECT_DESCRIPTION and a CODE_TEMPLATE for a website backend build
    /// Function: Takes an existing set of code marked as CODE_TEMPLATE and updates or re-writes it to work for the purpose in PROJECT_DESCRIPTION
    /// Important: The following libraries are already installed
    ///    reqwest, serde, serde_json, tokio, actix-web, async-trait, actix_cors
    /// Therefore, this function can only work with code from the standard Rust library or the above as per shown in the CODE_TEMPLATE
    /// Output: Print ONLY the code, nothing else. This function ONLY prints code.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_improved_webserrver_code(_project_description_and_template: &str) {
    /// Input: Takes in a PROJECT_DESCRIPTION and a CODE_TEMPLATE for a website backend build
    /// Function: Performs the following tasks:
    ///     1. Removes any bugs in the code and adds minor additional functionality
    ///     2. Makes sure everything requested in the spec from a backend standpoint was followed. If not, add the feature. No other features should be added.
    ///     3. ONLY writes the code. No commentary.
    /// Important: The following libraries are already installed. Does not use ANY libraries other than what was provided in the CODE_TEMPLATE
    ///     reqwest, serde, serde_json, tokio, actix-web, async-trait, actix_cors
    println!(OUTPUT)
}

#[ai_function]
pub fn print_fixed_code(_broken_code_with_bugs: &str) {
    /// Input: Takes in Rust BROKEN_CODE and the ERROR_BUGS found
    /// Function: Removes bugs from code
    /// Important: Only prints out the new and improved code. No commentary or anything else.
    println!(OUTPUT)
}

#[ai_function]
pub fn print_rest_api_endpoints(_code_input: &str) {
    /// Input: Takes in Rust webserver CODE_INPUT based on actix-web
    /// Function: Prints out the JSON schema for url endpoints and their respective types
    /// Logic: Script analyses all code and can categorize into the following object keys:
    ///     "route": This represents the url path of the endpoint
    ///     "is_route_dynamic": if a route has curly braces in it such as {symbol} or {id} as an example, then this will be set to true
    ///     "method": This represents the method being called
    ///     "request_body": This represents the body of a post method request
    ///     "response": This represents the output based upon the structs in the code and understanding the functions
    /// Important: Only prints out the JSON schema. No commentary or anything else.
    /// Must read: All keys are strings. Even bool should be wrapped in double quotes as "bool"
    /// Example:
    /// input_code:
    /// ...
    /// pub struct Item {
    ///     pub id: u64,
    ///     pub name: String,
    ///     pub completed: bool,
    /// }
    /// pub struct User {
    ///     pub id: u64,
    ///     pub username: String,
    ///     pub password: String,
    /// }
    /// ...
    /// HttpServer::new(move || {
    ///     App::new()
    ///         .app_data(data.clone())
    ///         .route("/item", web::post().to(create_item))
    ///         .route("/item/{id}", web::get().to(read_item))
    ///         .route("/item/{id}", web::put().to(update_item))
    ///         .route("/item/{id}", web::delete().to(delete_item))
    ///         .route("/signup", web::post().to(signup))
    ///         .route("/crypto", web::get().to(crypto))
    /// })
    /// Prints JSON formatted output:
    /// [
    ///     {
    ///         "route": "/item",
    ///         "is_route_dynamic": "false",
    ///         "method": "post",
    ///         "request_body": {
    ///             "id": "number",
    ///             "name": "string",
    ///             "completed": "bool",
    ///         },
    ///     },
    ///     {
    ///         "route": "/item/{id}",
    ///         "is_route_dynamic": "true",
    ///         "method": "get",
    ///         "request_body": "None",
    ///         "response": {
    ///             "id": "number",
    ///             "name": "string",
    ///             "completed": "bool",
    ///         }
    ///     },
    ///     {
    ///         "route": "/item/{id}",
    ///         "is_route_dynamic": "true",
    ///         "method": "delete",
    ///         "request_body": "None",
    ///         "response": "None"
    ///     },
    ///     {
    ///         "route": "/crypto",
    ///         "is_route_dynamic": "false",
    ///         "method": "get",
    ///         "request_body": "None",
    ///         "response": "not_provided"
    ///     },
    ///     ... // etc
    /// ]
    println!(OUTPUT)
}
