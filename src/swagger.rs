/// Generates swagger ui with specified url
pub fn generate_swagger(url: &str) -> String {
    include_str!("./swagger.html").replace("https://petstore3.swagger.io/api/v3/openapi.json", url)
}
