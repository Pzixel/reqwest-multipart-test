fn main() {
    use reqwest::blocking::multipart;

    let form = multipart::Form::new()
        // Adding just a simple text field...
        .text("username", "seanmonstar")
        // And a file...
        .file("photo", "/path/to/photo.png").unwrap();

// Customize all the details of a Part if needed...
    let bio = multipart::Part::text("hallo peeps")
        .file_name("bio.txt")
        .mime_str("text/plain").unwrap();

// Add the custom part to our form...
    let form = form.part("biography", bio);

// And finally, send the form
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post("http://localhost:8080/user")
        .multipart(form)
        .send().unwrap();
}
